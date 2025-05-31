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
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "Nullable",
        "postgresql_json_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "Nullable",
        "postgresql_json_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "Nullable"
            }
        }
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "Nullable",
        "postgresql_json_type_pattern": {
            "ArrayDimension4": {
                "dimension1_not_null_or_nullable": "Nullable",
                "dimension2_not_null_or_nullable": "Nullable",
                "dimension3_not_null_or_nullable": "Nullable",
                "dimension4_not_null_or_nullable": "Nullable"
            }
        }
    },
    {
        "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    }
    // ,
    // {
    //     "postgresql_json_type": "StdStringStringAsJsonbString",
    //     "not_null_or_nullable": "NotNull",
    //     "postgresql_json_type_pattern": "Standart"
    // }
    // {
    //     "postgresql_json_type": "UuidUuidAsJsonbString",
    //     "not_null_or_nullable": "NotNull",
    //     "postgresql_json_type_pattern": "Standart"
    // }
]);


/////////////
#[derive(Debug)]
pub struct StdStringStringAsNotNullJsonbString;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdStringStringAsNotNullJsonbStringOrigin(std::string::String);
impl StdStringStringAsNotNullJsonbStringOrigin {
    pub fn new(value: std::string::String) -> Self {
        Self(value)
    }
}
impl crate::IsStringEmpty for StdStringStringAsNotNullJsonbStringOrigin {
    fn is_string_empty(&self) -> std::primitive::bool {
        self.0.to_string().is_empty()
    }
}
impl std::fmt::Display for StdStringStringAsNotNullJsonbStringOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for StdStringStringAsNotNullJsonbStringOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdStringStringAsNotNullJsonbStringOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for StdStringStringAsNotNullJsonbStringOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<std::string::String> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<std::string::String> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdStringStringAsNotNullJsonbStringOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self.0.clone()), buf)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdStringStringAsNotNullJsonbStringSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdStringStringAsNotNullJsonbStringSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum StdStringStringAsNotNullJsonbStringWhereElement {
    Equal(crate::where_element_filters::PostgresqlJsonTypeWhereElementEqual<StdStringStringAsNotNullJsonbStringOrigin>),
    CaseSensitiveRegularExpression(crate::where_element_filters::PostgresqlJsonTypeWhereElementCaseSensitiveRegularExpression<StdStringStringAsNotNullJsonbStringOrigin>),
    CaseInsensitiveRegularExpression(crate::where_element_filters::PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<StdStringStringAsNotNullJsonbStringOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for StdStringStringAsNotNullJsonbStringWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CaseSensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CaseInsensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::CaseSensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::CaseInsensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdStringStringAsNotNullJsonbStringWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdStringStringAsNotNullJsonbStringWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CaseSensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CaseInsensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdStringStringAsNotNullJsonbStringRead(StdStringStringAsNotNullJsonbStringOrigin);
impl StdStringStringAsNotNullJsonbStringRead {
    pub fn new(value: std::string::String) -> Self {
        Self(StdStringStringAsNotNullJsonbStringOrigin::new(value))
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdStringStringAsNotNullJsonbStringRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
pub type StdStringStringAsNotNullJsonbStringReadInner = std::string::String;
impl crate::PostgresqlJsonType for StdStringStringAsNotNullJsonbString {
    type TableTypeDeclaration = StdStringStringAsNotNullJsonbStringOrigin;
    type Create = StdStringStringAsNotNullJsonbStringOrigin;
    fn create_query_part(_: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
    type Select = StdStringStringAsNotNullJsonbStringSelect;
    fn select_query_part(_: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',({column_name_and_maybe_field_getter}->'{field_ident}')))")
    }
    type WhereElement = StdStringStringAsNotNullJsonbStringWhereElement;
    type Read = StdStringStringAsNotNullJsonbStringRead;
    type ReadInner = StdStringStringAsNotNullJsonbStringReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0 .0
    }
    type Update = StdStringStringAsNotNullJsonbStringOrigin;
    fn update_query_part(_: &Self::Update, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
/////////////
/////////////
/////////////
/////////////
impl std::convert::TryFrom<StdStringStringAsNotNullJsonbStringOrigin> for UuidUuidAsNotNullJsonbStringOrigin {
    type Error = uuid::Error;
    fn try_from(value: StdStringStringAsNotNullJsonbStringOrigin) -> Result<Self, Self::Error> {
        match uuid::Uuid::parse_str(&value.0) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
////
#[derive(Debug)]
pub struct UuidUuidAsNotNullJsonbString;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UuidUuidAsNotNullJsonbStringOrigin(uuid::Uuid);
impl UuidUuidAsNotNullJsonbStringOrigin {
    pub fn new(value: uuid::Uuid) -> Self {
        Self(value)
    }
    pub fn query_bind_as_postgresql_text(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query.bind(self.0.to_string())
    }
    pub fn get_inner<'a>(&'a self) -> &'a uuid::Uuid {
        &self.0
    }
}
impl schemars::JsonSchema for UuidUuidAsNotNullJsonbStringOrigin {
    fn schema_name() -> std::string::String {
        "UuidUuidAsNotNullJsonbStringOrigin".to_owned()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("postgersql_crud::postgersql_json_type::UuidUuidAsNotNullJsonbStringOrigin")
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
        {
            schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                metadata: Some(Box::new(schemars::schema::Metadata {
                    id: None,
                    title: Some("UuidUuidAsNotNullJsonbStringOrigin".to_owned()),
                    description: None,
                    default: None,
                    deprecated: false,
                    read_only: false,
                    write_only: false,
                    examples: std::vec::Vec::default(),
                })),
                instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::String))),
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: None,
                string: Some(Box::new(schemars::schema::StringValidation {
                    max_length: Some(36),
                    min_length: Some(36),
                    pattern: None,
                })),
                array: None,
                object: None,
                reference: None,
                extensions: schemars::Map::default(),
            })
        }
    }
}
impl crate::IsStringEmpty for UuidUuidAsNotNullJsonbStringOrigin {
    fn is_string_empty(&self) -> std::primitive::bool {
        self.0.to_string().is_empty()
    }
}
impl std::fmt::Display for UuidUuidAsNotNullJsonbStringOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for UuidUuidAsNotNullJsonbStringOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
impl sqlx::Type<sqlx::Postgres> for UuidUuidAsNotNullJsonbStringOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<uuid::Uuid> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<uuid::Uuid> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UuidUuidAsNotNullJsonbStringOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self.0.clone()), buf)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum UuidUuidAsNotNullJsonbStringWhereElement {
    Equal(crate::where_element_filters::PostgresqlJsonTypeWhereElementEqual<UuidUuidAsNotNullJsonbStringOrigin>),
    CaseSensitiveRegularExpression(crate::where_element_filters::PostgresqlJsonTypeWhereElementCaseSensitiveRegularExpression<UuidUuidAsNotNullJsonbStringOrigin>),
    CaseInsensitiveRegularExpression(crate::where_element_filters::PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<UuidUuidAsNotNullJsonbStringOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for UuidUuidAsNotNullJsonbStringWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CaseSensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CaseInsensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::CaseSensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::CaseInsensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for UuidUuidAsNotNullJsonbStringWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CaseSensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CaseInsensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringRead(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringRead {
    pub fn new(value: uuid::Uuid) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
pub type UuidUuidAsNotNullJsonbStringReadInner = uuid::Uuid;
impl crate::PostgresqlJsonType for UuidUuidAsNotNullJsonbString {
    type TableTypeDeclaration = UuidUuidAsNotNullJsonbStringOrigin;
    type Create = UuidUuidAsNotNullJsonbStringOrigin;
    fn create_query_part(_: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
    type Select = UuidUuidAsNotNullJsonbStringSelect;
    fn select_query_part(_: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',({column_name_and_maybe_field_getter}->'{field_ident}')))")
    }
    type WhereElement = UuidUuidAsNotNullJsonbStringWhereElement;
    type Read = UuidUuidAsNotNullJsonbStringRead;
    type ReadInner = UuidUuidAsNotNullJsonbStringReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0 .0
    }
    type Update = UuidUuidAsNotNullJsonbStringOrigin;
    fn update_query_part(_: &Self::Update, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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


// #[derive(Debug)]
// pub struct UuidUuidAsNotNullJsonbString;
// #[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct UuidUuidAsNotNullJsonbStringOrigin(uuid::Uuid);
// impl UuidUuidAsNotNullJsonbStringOrigin {
//     pub fn new(value: uuid::Uuid) -> Self {
//         Self(value)
//     }
//     pub fn query_bind_as_postgresql_text(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query.bind(self.0.to_string())
//     }
//     pub fn get_inner<'a>(&'a self) -> &'a uuid::Uuid {
//         &self.0
//     }
// }
// impl schemars::JsonSchema for UuidUuidAsNotNullJsonbStringOrigin {
//     fn schema_name() -> std::string::String {
//         "UuidUuidAsNotNullJsonbStringOrigin".to_owned()
//     }
//     fn schema_id() -> std::borrow::Cow<'static, str> {
//         std::borrow::Cow::Borrowed("postgersql_crud::postgersql_json_type::UuidUuidAsNotNullJsonbStringOrigin")
//     }
//     fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
//         {
//             schemars::schema::Schema::Object(schemars::schema::SchemaObject {
//                 metadata: Some(Box::new(schemars::schema::Metadata {
//                     id: None,
//                     title: Some("UuidUuidAsNotNullJsonbStringOrigin".to_owned()),
//                     description: None,
//                     default: None,
//                     deprecated: false,
//                     read_only: false,
//                     write_only: false,
//                     examples: std::vec::Vec::default(),
//                 })),
//                 instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::String))),
//                 format: None,
//                 enum_values: None,
//                 const_value: None,
//                 subschemas: None,
//                 number: None,
//                 string: Some(Box::new(schemars::schema::StringValidation {
//                     max_length: Some(36),
//                     min_length: Some(36),
//                     pattern: None,
//                 })),
//                 array: None,
//                 object: None,
//                 reference: None,
//                 extensions: schemars::Map::default(),
//             })
//         }
//     }
// }
// impl crate::IsStringEmpty for UuidUuidAsNotNullJsonbStringOrigin {
//     fn is_string_empty(&self) -> std::primitive::bool {
//         self.0.to_string().is_empty()
//     }
// }
// impl std::fmt::Display for UuidUuidAsNotNullJsonbStringOrigin {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{self:?}")
//     }
// }
// impl error_occurence_lib::ToStdStringString for UuidUuidAsNotNullJsonbStringOrigin {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringOrigin {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(uuid::Uuid::new_v4())
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for UuidUuidAsNotNullJsonbStringOrigin {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <sqlx::types::Json<uuid::Uuid> as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <sqlx::types::Json<uuid::Uuid> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl sqlx::Encode<'_, sqlx::Postgres> for UuidUuidAsNotNullJsonbStringOrigin {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self.0.clone()), buf)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct UuidUuidAsNotNullJsonbStringSelect;
// impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringSelect {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self {}
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
// pub enum UuidUuidAsNotNullJsonbStringWhereElement {
//     Equal(crate::where_element_filters::PostgresqlJsonTypeWhereElementEqual<UuidUuidAsNotNullJsonbStringOrigin>),
//     CaseSensitiveRegularExpression(crate::where_element_filters::PostgresqlJsonTypeWhereElementCaseSensitiveRegularExpression<UuidUuidAsNotNullJsonbStringOrigin>),
//     CaseInsensitiveRegularExpression(crate::where_element_filters::PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<UuidUuidAsNotNullJsonbStringOrigin>),
// }
// impl<'a> crate::PostgresqlTypeWhereFilter<'a> for UuidUuidAsNotNullJsonbStringWhereElement {
//     fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
//         match &self {
//             Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::CaseSensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::CaseInsensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//         }
//     }
//     fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         match self {
//             Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::CaseSensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::CaseInsensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
//         }
//     }
// }
// impl error_occurence_lib::ToStdStringString for UuidUuidAsNotNullJsonbStringWhereElement {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringWhereElement {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         vec![
//             Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::CaseSensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::CaseInsensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//         ]
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct UuidUuidAsNotNullJsonbStringRead(UuidUuidAsNotNullJsonbStringOrigin);
// impl UuidUuidAsNotNullJsonbStringRead {
//     pub fn new(value: uuid::Uuid) -> Self {
//         Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
//     }
// }
// impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringRead {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
//     }
// }
// pub type UuidUuidAsNotNullJsonbStringReadInner = uuid::Uuid;
// impl crate::PostgresqlJsonType for UuidUuidAsNotNullJsonbString {
//     type TableTypeDeclaration = UuidUuidAsNotNullJsonbStringOrigin;
//     type Create = UuidUuidAsNotNullJsonbStringOrigin;
//     fn create_query_part(_: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
//         match increment.checked_add(1) {
//             Some(value) => {
//                 *increment = value;
//                 Ok(format!("${increment}"))
//             }
//             None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn create_query_bind(value: Self::Create, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(value);
//         query
//     }
//     type Select = UuidUuidAsNotNullJsonbStringSelect;
//     fn select_query_part(_: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
//         format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',({column_name_and_maybe_field_getter}->'{field_ident}')))")
//     }
//     type WhereElement = UuidUuidAsNotNullJsonbStringWhereElement;
//     type Read = UuidUuidAsNotNullJsonbStringRead;
//     type ReadInner = UuidUuidAsNotNullJsonbStringReadInner;
//     fn into_inner(value: Self::Read) -> Self::ReadInner {
//         value.0 .0
//     }
//     type Update = UuidUuidAsNotNullJsonbStringOrigin;
//     fn update_query_part(_: &Self::Update, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
//         match increment.checked_add(1) {
//             Some(value) => {
//                 *increment = value;
//                 Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
//             }
//             None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn update_query_bind(value: Self::Update, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(value);
//         query
//     }
// }



