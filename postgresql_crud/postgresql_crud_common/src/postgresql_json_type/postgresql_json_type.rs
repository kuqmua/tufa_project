postgresql_crud_types_macro_logic_reuse::generate_postgresql_json_types!();

/////////////////////////////
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct Uuid(pub uuid::Uuid);
//here +
impl schemars::JsonSchema for Uuid {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("Uuid")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::Uuid")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        {
            let mut schema = generator.subschema_for::<std::string::String>();
            schemars::_private::insert_validation_property(&mut schema, "string", "minLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "string", "maxLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "minItems", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "maxItems", 36);
            schema
        }
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for Uuid {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        //maybe compile time uuid gen? not need to gen on runtime. if all different on compile time than its enough 
        Self(uuid::Uuid::new_v4())
    }
}
impl error_occurence_lib::ToStdStringString for Uuid {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}

pub type PostgresqlJsonTypeUuidToCreate = Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeUuidFieldReader {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidFieldReader {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}

pub type PostgresqlJsonTypeUuidOptionsToRead = Uuid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct PostgresqlJsonTypeUuidWhere {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<PostgresqlJsonTypeUuidWhereElement>,
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeUuidWhere {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for element in &self.value {
            match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                Ok(value) => {
                    acc.push_str(&format!("{value} "));
                    is_need_to_add_logical_operator_inner_handle = true;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _ = acc.pop();
        Ok(format!("{}({acc})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.value {
            query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
        }
        query
    }
}
//todo impl try_new + custom serde::Deserialize
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidWhere {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}

























#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostgresqlJsonTypeUuidWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: uuid::Uuid,
}
impl schemars::JsonSchema for PostgresqlJsonTypeUuidWhereElementEqual {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("PostgresqlJsonTypeUuidWhereElementEqual")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("server::entry::PostgresqlJsonTypeUuidWhereElementEqual")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        {
            let mut schema = ::schemars::Schema::try_from(
                ::serde_json::Value::Object({
                    let mut object = ::serde_json::Map::new();
                    let _ = object.insert(
                        ("type").into(),
                        ::serde_json::to_value(&"object").unwrap(),
                    );
                    object
                }),
            ).unwrap();
            {
                schemars::_private::insert_object_property::<crate::LogicalOperator>(
                    &mut schema,
                    "logical_operator",
                    false,
                    false,
                    { generator.subschema_for::<crate::LogicalOperator>() },
                );
            }
            {
                schemars::_private::insert_object_property::<Uuid>(
                    &mut schema,
                    "value",
                    false,
                    false,
                    { generator.subschema_for::<Uuid>() },
                );
            }
            schema
        }
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidWhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeUuidWhereElementEqual {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}

#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars::JsonSchema)]
pub enum PostgresqlJsonTypeUuidWhereElement {
    Equal(PostgresqlJsonTypeUuidWhereElementEqual),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeUuidWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlJsonTypeUuidWhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlJsonTypeUuidWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeUuidWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}

pub type PostgresqlJsonTypeUuidOptionToUpdate = Uuid;

#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeUuidOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}

impl crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonType for Uuid {
    type PostgresqlJsonTypeSelfToCreate<'a> = PostgresqlJsonTypeUuidToCreate;
    fn try_generate_postgresql_json_type_to_create(_: &Self::PostgresqlJsonTypeSelfToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(format!("${increment}"))
            }
            None => Err(crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(postgresql_json_type_self_to_create: Self::PostgresqlJsonTypeSelfToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(postgresql_json_type_self_to_create.0));
        query
    }
    type PostgresqlJsonTypeSelfFieldReader<'a> = PostgresqlJsonTypeUuidFieldReader;
    type PostgresqlJsonTypeSelfOptionsToRead<'a> = PostgresqlJsonTypeUuidOptionsToRead;
    fn generate_postgresql_json_type_to_read(
        _: &Self::PostgresqlJsonTypeSelfFieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        _: &std::primitive::str,
        is_postgresql_type: std::primitive::bool,
    ) -> std::string::String {
        //todo refactor is_empty
        let column_name_and_maybe_field_getter_handle = if column_name_and_maybe_field_getter.is_empty() {
            column_name_and_maybe_field_getter.to_string()
        }
        else {
            format!("{column_name_and_maybe_field_getter}->")
        };
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter_handle}'{field_ident}'))")
    }
    type PostgresqlJsonTypeSelfWhereElement<'a> = PostgresqlJsonTypeUuidWhereElement;
    type PostgresqlJsonTypeSelfWhere = PostgresqlJsonTypeUuidWhere;
    type PostgresqlJsonTypeSelfOptionToUpdate<'a> = PostgresqlJsonTypeUuidOptionToUpdate;
    type PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed = PostgresqlJsonTypeUuidOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed;
    fn try_generate_postgresql_json_type_to_update(
        _: &Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        _: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(postgresql_json_type_self_option_to_update: Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(postgresql_json_type_self_option_to_update.0));
        query
    }
}