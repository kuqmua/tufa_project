// generate_postgresql_json_types::generate_postgresql_json_types!();

#[derive(Debug, Clone, PartialEq, PartialOrd, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8(pub std::primitive::i8);
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8 {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI8 {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
pub type StdPrimitiveI8Create = StdPrimitiveI8;
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8Select {}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8Select {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
pub type StdPrimitiveI8Read = StdPrimitiveI8;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum StdPrimitiveI8WhereElement {
    Equal(crate::where_element_filters::PostgresqlJsonTypeWhereElementEqual<StdPrimitiveI8>),
    GreaterThan(crate::where_element_filters::PostgresqlJsonTypeWhereElementGreaterThan<StdPrimitiveI8>),
    Between(crate::where_element_filters::PostgresqlJsonTypeWhereElementBetween<StdPrimitiveI8>),
    In(crate::where_element_filters::PostgresqlJsonTypeWhereElementIn<StdPrimitiveI8>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for StdPrimitiveI8WhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::In(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(
        self,
        // mut
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::In(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI8WhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8WhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::In(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
pub type StdPrimitiveI8Update = StdPrimitiveI8;
impl crate::PostgresqlJsonType for StdPrimitiveI8 {
    type Create = StdPrimitiveI8Create;
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
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
    type Select = StdPrimitiveI8Select;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter}->'{field_ident}'))")
    }
    type WhereElement = StdPrimitiveI8WhereElement;
    type Read = StdPrimitiveI8Read;
    type Update = StdPrimitiveI8Update;
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
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UuidUuid(pub uuid::Uuid);
impl schemars::JsonSchema for UuidUuid {
    fn schema_name() -> std::string::String {
        "UuidUuid".to_owned()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("postgersql_crud::postgersql_json_type::UuidUuid")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
        {
            schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                metadata: Some(Box::new(schemars::schema::Metadata {
                    id: None,
                    title: Some("UuidUuid".to_owned()),
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
impl crate::IsEmpty for UuidUuid {
    fn is_empty(&self) -> std::primitive::bool {
        self.0.to_string().is_empty()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuid {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
impl error_occurence_lib::ToStdStringString for UuidUuid {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
pub type UuidUuidCreate = UuidUuid;
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidSelect {}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
pub type UuidUuidRead = UuidUuid;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum UuidUuidWhereElement {
    Equal(crate::where_element_filters::PostgresqlJsonTypeWhereElementEqual<UuidUuid>),
    CaseSensitiveRegularExpression(crate::where_element_filters::PostgresqlJsonTypeWhereElementCaseSensitiveRegularExpression<UuidUuid>),
    CaseInsensitiveRegularExpression(crate::where_element_filters::PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<UuidUuid>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for UuidUuidWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CaseSensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CaseInsensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(
        self,
        // mut
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::CaseSensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::CaseInsensitiveRegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for UuidUuidWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CaseSensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CaseInsensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
pub type UuidUuidUpdate = UuidUuid;
impl crate::PostgresqlJsonType for UuidUuid {
    type Create = UuidUuidCreate;
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
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
    type Select = UuidUuidSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter}->'{field_ident}'))")
    }
    type WhereElement = UuidUuidWhereElement;
    type Read = UuidUuidRead;
    type Update = UuidUuidUpdate;
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
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
}
