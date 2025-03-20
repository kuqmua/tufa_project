generate_postgresql_json_types::generate_postgresql_json_types!();

#[derive(Debug, Clone, PartialEq, PartialOrd, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdVecVecStdVecVecUuidUuid(pub std::vec::Vec<std::vec::Vec<UuidUuid>>);
impl crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecStdVecVecUuidUuid {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![vec![
            crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        ]])
    }
}
impl error_occurence_lib::ToStdStringString for StdVecVecStdVecVecUuidUuid {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
pub type StdVecVecStdVecVecUuidUuidCreate = StdVecVecStdVecVecUuidUuid;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdVecVecStdVecVecUuidUuidSelect {
    //todo could not implement multi dimension pagination
    pagination: crate::pagination::Pagination,
}
impl crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecStdVecVecUuidUuidSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            pagination: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
pub type StdVecVecStdVecVecUuidUuidRead = StdVecVecStdVecVecUuidUuid;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct StdVecVecStdVecVecUuidUuidWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: StdVecVecStdVecVecUuidUuid,
}
impl crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecStdVecVecUuidUuidWhereElementEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'_> for StdVecVecStdVecVecUuidUuidWhereElementEqual {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}

#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum StdVecVecStdVecVecUuidUuidWhereElement {
    Equal(StdVecVecStdVecVecUuidUuidWhereElementEqual),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'_> for StdVecVecStdVecVecUuidUuidWhereElement {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn where_query_bind<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdVecVecStdVecVecUuidUuidWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecStdVecVecUuidUuidWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Equal(
            crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        )]
    }
}
pub type StdVecVecStdVecVecUuidUuidUpdate = StdVecVecStdVecVecUuidUuid;
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdVecVecStdVecVecUuidUuidOptionToUpdateTryGenerateErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonType for StdVecVecStdVecVecUuidUuid {
    type Create<'a> = StdVecVecStdVecVecUuidUuidCreate;
    fn create_query_part(_: &Self::Create<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::postgresql_json_type::postgresql_json_type_trait::CreateQueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::postgresql_json_type::postgresql_json_type_trait::CreateQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn create_query_bind<'a>(value: Self::Create<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
    type Select<'a> = StdVecVecStdVecVecUuidUuidSelect;
    type Read<'a> = StdVecVecStdVecVecUuidUuidRead;
    fn select_query_part(value: &Self::Select<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        // println!("fn select_query_part");
        //todo change
        let start = value.pagination.start();
        let end = value.pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end})))")
    }
    type WhereElement<'a> = StdVecVecStdVecVecUuidUuidWhereElement;
    type Update<'a> = StdVecVecStdVecVecUuidUuidUpdate;
    type UpdateQueryPartErrorNamed = StdVecVecStdVecVecUuidUuidOptionToUpdateTryGenerateErrorNamed;
    fn update_query_part(_: &Self::Update<'_>, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, Self::UpdateQueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::UpdateQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn update_query_bind<'a>(value: Self::Update<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0)); //todo remove .0 and impl Encode instead
        query
    }
}