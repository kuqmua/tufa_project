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
struct AnimalToCreateOrigin {
    std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8ToCreate,
    std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8ToCreate,
    std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdPrimitiveI8ToCreate,
    std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8ToCreate,
    std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
    std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalToCreateOrigin {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl AnimalToCreateOrigin {
    fn try_generate_postgresql_query_part_to_create(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_vec_vec_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_vec_vec_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_option_option_std_vec_vec_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_vec_vec_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_vec_vec_std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_vec_vec_std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_option_option_std_vec_vec_std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_vec_vec_std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_option_option_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_vec_vec_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_option_option_std_vec_vec_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_vec_vec_std_option_option_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_option_option_std_vec_vec_std_option_option_std_primitive_i8, query);
        query
    }
}
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
pub struct AnimalToCreateWithGeneratedId(AnimalToCreateOrigin);
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
pub struct AnimalToCreateWithoutGeneratedId(AnimalToCreateOrigin);
impl AnimalToCreateWithGeneratedId {
    pub fn new(
        std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8ToCreate,
        std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8ToCreate,
        std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdPrimitiveI8ToCreate,
        std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8ToCreate,
        std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
        std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
    ) -> Self {
        Self(AnimalToCreateOrigin {
            std_primitive_i8,
            std_option_option_std_primitive_i8,
            std_vec_vec_std_primitive_i8,
            std_option_option_std_vec_vec_std_primitive_i8,
            std_vec_vec_std_option_option_std_primitive_i8,
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        })
    }
}
impl AnimalToCreateWithoutGeneratedId {
    pub fn new(
        std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8ToCreate,
        std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8ToCreate,
        std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdPrimitiveI8ToCreate,
        std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8ToCreate,
        std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
        std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
    ) -> Self {
        Self(AnimalToCreateOrigin {
            std_primitive_i8,
            std_option_option_std_primitive_i8,
            std_vec_vec_std_primitive_i8,
            std_option_option_std_vec_vec_std_primitive_i8,
            std_vec_vec_std_option_option_std_primitive_i8,
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        })
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalToCreateWithGeneratedId {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalToCreateWithoutGeneratedId {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl AnimalToCreateWithGeneratedId {
    fn try_generate_postgresql_query_part_to_create(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        match self.0.try_generate_postgresql_query_part_to_create(increment) {
            Ok(value) => Ok(format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{value}")),
            Err(error) => Err(error),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.0.bind_value_to_postgresql_query_part_to_create(query);
        query
    }
}
impl AnimalToCreateWithoutGeneratedId {
    fn try_generate_postgresql_query_part_to_create(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        match self.0.try_generate_postgresql_query_part_to_create(increment) {
            Ok(value) => Ok(value),
            Err(error) => Err(error),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.0.bind_value_to_postgresql_query_part_to_create(query);
        query
    }
}
impl<'a> postgresql_crud::BindQuery<'a> for AnimalToCreateWithoutGeneratedId {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_vec_vec_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_vec_vec_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_option_option_std_vec_vec_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_vec_vec_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_vec_vec_std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_vec_vec_std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_option_option_std_vec_vec_std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_vec_vec_std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_option_option_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_vec_vec_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_option_option_std_vec_vec_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_vec_vec_std_option_option_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_option_option_std_vec_vec_std_option_option_std_primitive_i8, query);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToReadWithoutId {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::json_types::StdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
    StdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
    StdVecVecStdPrimitiveI8(postgresql_crud::json_types::StdVecVecStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
    StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToReadWithId {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id(postgresql_crud::json_types::UuidFieldReader),
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::json_types::StdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
    StdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
    StdVecVecStdPrimitiveI8(postgresql_crud::json_types::StdVecVecStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
    StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToReadWithoutId {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalFieldToReadWithoutId {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalFieldToReadWithoutId::StdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithoutId::StdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithoutId::StdVecVecStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithoutId::StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToReadWithId {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalFieldToReadWithId {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalFieldToReadWithId::Id(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdVecVecStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct AnimalOptionsToReadWithoutId {
    #[serde(skip_serializing_if = "Option::is_none")]
    std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct AnimalOptionsToReadWithId {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::UuidOptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl AnimalOptionsToReadWithoutId {
    pub fn try_new(
        std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>,
        std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>,
        std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead>>,
        std_option_option_std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>,
        std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
        std_option_option_std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    ) -> Result<Self, AnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed> {
        if let (None, None, None, None, None, None) = (
            &std_primitive_i8,
            &std_option_option_std_primitive_i8,
            &std_vec_vec_std_primitive_i8,
            &std_option_option_std_vec_vec_std_primitive_i8,
            &std_vec_vec_std_option_option_std_primitive_i8,
            &std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        ) {
            return Err(AnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self {
            std_primitive_i8,
            std_option_option_std_primitive_i8,
            std_vec_vec_std_primitive_i8,
            std_option_option_std_vec_vec_std_primitive_i8,
            std_vec_vec_std_option_option_std_primitive_i8,
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        })
    }
}
impl AnimalOptionsToReadWithId {
    pub fn try_new(
        id: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::UuidOptionsToRead>>,
        std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>,
        std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>,
        std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead>>,
        std_option_option_std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>,
        std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
        std_option_option_std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    ) -> Result<Self, AnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed> {
        if let (None, None, None, None, None, None, None) = (
            &id,
            &std_primitive_i8,
            &std_option_option_std_primitive_i8,
            &std_vec_vec_std_primitive_i8,
            &std_option_option_std_vec_vec_std_primitive_i8,
            &std_vec_vec_std_option_option_std_primitive_i8,
            &std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        ) {
            return Err(AnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self {
            id,
            std_primitive_i8,
            std_option_option_std_primitive_i8,
            std_vec_vec_std_primitive_i8,
            std_option_option_std_vec_vec_std_primitive_i8,
            std_vec_vec_std_option_option_std_primitive_i8,
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        })
    }
}
impl<'de> serde::Deserialize<'de> for AnimalOptionsToReadWithoutId {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __field3,
            __field4,
            __field5,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    3u64 => serde::__private::Ok(__Field::__field3),
                    4u64 => serde::__private::Ok(__Field::__field4),
                    5u64 => serde::__private::Ok(__Field::__field5),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "std_primitive_i8" => serde::__private::Ok(__Field::__field0),
                    "std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field1),
                    "std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field2),
                    "std_option_option_std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field3),
                    "std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field4),
                    "std_option_option_std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field5),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"std_primitive_i8" => serde::__private::Ok(__Field::__field0),
                    b"std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field1),
                    b"std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field2),
                    b"std_option_option_std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field3),
                    b"std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field4),
                    b"std_option_option_std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field5),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<AnimalOptionsToReadWithoutId>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = AnimalOptionsToReadWithoutId;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct AnimalOptionsToRead")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 6 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 6 elements"));
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 6 elements"));
                    }
                };
                let __field3 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 6 elements"));
                    }
                };
                let __field4 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 6 elements"));
                    }
                };
                let __field5 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 6 elements"));
                    }
                };
                match AnimalOptionsToReadWithoutId::try_new(__field0, __field1, __field2, __field3, __field4, __field5) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field3: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field4: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field5: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i8"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_primitive_i8"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_std_primitive_i8"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field3 => {
                            if serde::__private::Option::is_some(&__field3) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_std_primitive_i8"));
                            }
                            __field3 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field4 => {
                            if serde::__private::Option::is_some(&__field4) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_std_option_option_std_primitive_i8"));
                            }
                            __field4 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field5 => {
                            if serde::__private::Option::is_some(&__field5) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"));
                            }
                            __field5 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(
                                &mut __map,
                            )?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("std_primitive_i8")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_primitive_i8")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("std_vec_vec_std_primitive_i8")?,
                };
                let __field3 = match __field3 {
                    serde::__private::Some(__field3) => __field3,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_std_primitive_i8")?,
                };
                let __field4 = match __field4 {
                    serde::__private::Some(__field4) => __field4,
                    serde::__private::None => serde::__private::de::missing_field("std_vec_vec_std_option_option_std_primitive_i8")?,
                };
                let __field5 = match __field5 {
                    serde::__private::Some(__field5) => __field5,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8")?,
                };
                match AnimalOptionsToReadWithoutId::try_new(__field0, __field1, __field2, __field3, __field4, __field5) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &[
            "std_primitive_i8",
            "std_option_option_std_primitive_i8",
            "std_vec_vec_std_primitive_i8",
            "std_option_option_std_vec_vec_std_primitive_i8",
            "std_vec_vec_std_option_option_std_primitive_i8",
            "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
        ];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "AnimalOptionsToReadWithoutId",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<AnimalOptionsToReadWithoutId>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl<'de> serde::Deserialize<'de> for AnimalOptionsToReadWithId {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __field3,
            __field4,
            __field5,
            __field6,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    3u64 => serde::__private::Ok(__Field::__field3),
                    4u64 => serde::__private::Ok(__Field::__field4),
                    5u64 => serde::__private::Ok(__Field::__field5),
                    6u64 => serde::__private::Ok(__Field::__field6),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "id" => serde::__private::Ok(__Field::__field0),
                    "std_primitive_i8" => serde::__private::Ok(__Field::__field1),
                    "std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field2),
                    "std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field3),
                    "std_option_option_std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field4),
                    "std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field5),
                    "std_option_option_std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field6),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"id" => serde::__private::Ok(__Field::__field0),
                    b"std_primitive_i8" => serde::__private::Ok(__Field::__field1),
                    b"std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field2),
                    b"std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field3),
                    b"std_option_option_std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field4),
                    b"std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field5),
                    b"std_option_option_std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field6),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<AnimalOptionsToReadWithId>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = AnimalOptionsToReadWithId;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct AnimalOptionsToRead")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::UuidOptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field3 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field4 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field5 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field6 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalOptionsToRead with 7 elements"));
                    }
                };
                match AnimalOptionsToReadWithId::try_new(__field0, __field1, __field2, __field3, __field4, __field5, __field6) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::UuidOptionsToRead>>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field3: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field4: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field5: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field6: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("id"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::UuidOptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i8"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_primitive_i8"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field3 => {
                            if serde::__private::Option::is_some(&__field3) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_std_primitive_i8"));
                            }
                            __field3 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field4 => {
                            if serde::__private::Option::is_some(&__field4) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_std_primitive_i8"));
                            }
                            __field4 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field5 => {
                            if serde::__private::Option::is_some(&__field5) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_std_option_option_std_primitive_i8"));
                            }
                            __field5 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field6 => {
                            if serde::__private::Option::is_some(&__field6) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"));
                            }
                            __field6 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(
                                &mut __map,
                            )?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("id")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("std_primitive_i8")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_primitive_i8")?,
                };
                let __field3 = match __field3 {
                    serde::__private::Some(__field3) => __field3,
                    serde::__private::None => serde::__private::de::missing_field("std_vec_vec_std_primitive_i8")?,
                };
                let __field4 = match __field4 {
                    serde::__private::Some(__field4) => __field4,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_std_primitive_i8")?,
                };
                let __field5 = match __field5 {
                    serde::__private::Some(__field5) => __field5,
                    serde::__private::None => serde::__private::de::missing_field("std_vec_vec_std_option_option_std_primitive_i8")?,
                };
                let __field6 = match __field6 {
                    serde::__private::Some(__field6) => __field6,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8")?,
                };
                match AnimalOptionsToReadWithId::try_new(__field0, __field1, __field2, __field3, __field4, __field5, __field6) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &[
            "id",
            "std_primitive_i8",
            "std_option_option_std_primitive_i8",
            "std_vec_vec_std_primitive_i8",
            "std_option_option_std_vec_vec_std_primitive_i8",
            "std_vec_vec_std_option_option_std_primitive_i8",
            "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
        ];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "AnimalOptionsToReadWithId",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<AnimalOptionsToReadWithId>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalOptionsToReadWithoutId {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_vec_vec_std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        }
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalOptionsToReadWithId {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            id: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_vec_vec_std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToUpdate {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8,
    #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
    StdOptionOptionStdPrimitiveI8,
    #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
    StdVecVecStdPrimitiveI8,
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdPrimitiveI8,
    #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
    StdVecVecStdOptionOptionStdPrimitiveI8,
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        match &self {
            Self::StdPrimitiveI8 => "std_primitive_i8".to_owned(),
            Self::StdOptionOptionStdPrimitiveI8 => "std_option_option_std_primitive_i8".to_owned(),
            Self::StdVecVecStdPrimitiveI8 => "std_vec_vec_std_primitive_i8".to_owned(),
            Self::StdOptionOptionStdVecVecStdPrimitiveI8 => "std_option_option_std_vec_vec_std_primitive_i8".to_owned(),
            Self::StdVecVecStdOptionOptionStdPrimitiveI8 => "std_vec_vec_std_option_option_std_primitive_i8".to_owned(),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => "std_option_option_std_vec_vec_std_option_option_std_primitive_i8".to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalOptionToUpdateOrigin {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionToUpdate>),
    #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
    StdOptionOptionStdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionToUpdate>),
    #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
    StdVecVecStdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionToUpdate>),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionToUpdate>),
    #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
    StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdate>),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdate>),
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Create {
        #[eo_error_occurence]
        error: postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalOptionToUpdateOrigin {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalOptionToUpdateOrigin::StdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            AnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            AnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            AnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalOptionsToUpdate {
    pub id: postgresql_crud::json_types::UuidOptionToUpdate,
    pub fields: AnimalOptionToUpdate,
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalOptionsToUpdate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            fields: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl std::fmt::Display for Animal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", &self)
    }
}
pub type AnimalToCreate = AnimalToCreateWithoutGeneratedId;
pub type AnimalFieldToRead = AnimalFieldToReadWithoutId;
pub type AnimalOptionsToRead = AnimalOptionsToReadWithoutId;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct AnimalFieldReader(std::vec::Vec<AnimalFieldToReadWithoutId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalFieldReaderTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithoutId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl AnimalFieldReader {
    pub fn try_new(value: std::vec::Vec<AnimalFieldToReadWithoutId>) -> Result<Self, AnimalFieldReaderTryNewErrorNamed> {
        if value.is_empty() {
            return Err(AnimalFieldReaderTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(AnimalFieldReaderTryNewErrorNamed::NotUniqueFieldFilter {
                    field: element,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            } else {
                unique.push(element);
            }
        }
        Ok(Self(unique))
    }
}
impl<'de> serde::Deserialize<'de> for AnimalFieldReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<AnimalFieldReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = AnimalFieldReader;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct AnimalFieldReader")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalFieldToReadWithoutId> = <std::vec::Vec<AnimalFieldToReadWithoutId> as serde::Deserialize>::deserialize(__e)?;
                match AnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalFieldToReadWithoutId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct AnimalFieldReader with 1 element"));
                    }
                };
                match AnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "AnimalFieldReader",
            __Visitor {
                marker: serde::__private::PhantomData::<AnimalFieldReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalFieldReader {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl postgresql_crud::GeneratePostgresqlQueryPartToRead for AnimalFieldToReadWithoutId {
    fn generate_postgresql_query_part_to_read_from_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
        let mut acc = std::string::String::default();
        for element in value {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithoutId::StdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_primitive_i8", column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_option_option_std_primitive_i8", column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message),
                    AnimalFieldToReadWithoutId::StdVecVecStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_vec_vec_std_primitive_i8", column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_primitive_i8",
                        column_name_and_maybe_field_getter,
                        column_name_and_maybe_field_getter_for_error_message
                    ),
                    AnimalFieldToReadWithoutId::StdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_option_option_std_primitive_i8",
                        column_name_and_maybe_field_getter,
                        column_name_and_maybe_field_getter_for_error_message
                    ),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                        column_name_and_maybe_field_getter,
                        column_name_and_maybe_field_getter_for_error_message
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        format!("{acc}")
    }
}
pub type AnimalReader = AnimalOptionsToRead;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct AnimalOptionToUpdate(std::vec::Vec<AnimalOptionToUpdateOrigin>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalOptionToUpdateTryNewErrorNamed {
    FieldsAreEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdOptionOptionStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdVecVecStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdOptionOptionStdVecVecStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl AnimalOptionToUpdate {
    pub fn try_new(value: std::vec::Vec<AnimalOptionToUpdateOrigin>) -> Result<Self, AnimalOptionToUpdateTryNewErrorNamed> {
        if value.is_empty() {
            return Err(AnimalOptionToUpdateTryNewErrorNamed::FieldsAreEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            let generate_not_unique_field = |value: &std::primitive::str| format!("not unique {value} field");
            for element in &value {
                match element {
                    AnimalOptionToUpdateOrigin::StdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(AnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdPrimitiveI8 {
                                error: generate_not_unique_field("std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                    AnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdOptionOptionStdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(AnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdOptionOptionStdPrimitiveI8 {
                                error: generate_not_unique_field("std_option_option_std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                    AnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdVecVecStdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(AnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdVecVecStdPrimitiveI8 {
                                error: generate_not_unique_field("std_vec_vec_std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                    AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdOptionOptionStdVecVecStdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(AnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdOptionOptionStdVecVecStdPrimitiveI8 {
                                error: generate_not_unique_field("std_option_option_std_vec_vec_std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                    AnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdVecVecStdOptionOptionStdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(AnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdVecVecStdOptionOptionStdPrimitiveI8 {
                                error: generate_not_unique_field("std_vec_vec_std_option_option_std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                    AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(AnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
                                error: generate_not_unique_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                }
            }
        }
        Ok(Self(value))
    }
}
impl<'de> serde::Deserialize<'de> for AnimalOptionToUpdate {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<AnimalOptionToUpdate>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = AnimalOptionToUpdate;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct AnimalOptionToUpdate")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalOptionToUpdateOrigin> = <std::vec::Vec<AnimalOptionToUpdateOrigin> as serde::Deserialize>::deserialize(__e)?;
                match AnimalOptionToUpdate::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalOptionToUpdateOrigin>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct AnimalOptionToUpdate with 1 element"));
                    }
                };
                match AnimalOptionToUpdate::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "AnimalOptionToUpdate",
            __Visitor {
                marker: serde::__private::PhantomData::<AnimalOptionToUpdate>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalOptionToUpdate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl AnimalOptionToUpdate {
    fn try_generate_postgresql_query_part_to_update(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, AnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        let generate_jsonb_set_path = |value: &std::primitive::str| {
            let previous = match jsonb_set_path.is_empty() {
                true => std::string::String::default(),
                false => format!("{jsonb_set_path},"),
            };
            format!("{previous}{value}")
        };
        let mut local_acc = format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end)");
        for element in &self.0 {
            match &element {
                AnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                    match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(&value.value, &local_acc, &generate_jsonb_set_target("std_primitive_i8"), &generate_jsonb_set_path("std_primitive_i8"), increment) {
                        Ok(value) => {
                            local_acc = value;
                        }
                        Err(error) => {
                            return Err(AnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdPrimitiveI8 {
                                error,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                }
                AnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                    &value.value,
                    &local_acc,
                    &generate_jsonb_set_target("std_option_option_std_primitive_i8"),
                    &generate_jsonb_set_path("std_option_option_std_primitive_i8"),
                    increment,
                ) {
                    Ok(value) => {
                        local_acc = value;
                    }
                    Err(error) => {
                        return Err(AnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdPrimitiveI8 {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                },
                AnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                    &value.value,
                    &local_acc,
                    &generate_jsonb_set_target("std_vec_vec_std_primitive_i8"),
                    &generate_jsonb_set_path("std_vec_vec_std_primitive_i8"),
                    increment,
                ) {
                    Ok(value) => {
                        local_acc = value;
                    }
                    Err(error) => {
                        return Err(AnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdPrimitiveI8 {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                },
                AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                    &value.value,
                    &local_acc,
                    &generate_jsonb_set_target("std_option_option_std_vec_vec_std_primitive_i8"),
                    &generate_jsonb_set_path("std_option_option_std_vec_vec_std_primitive_i8"),
                    increment,
                ) {
                    Ok(value) => {
                        local_acc = value;
                    }
                    Err(error) => {
                        return Err(AnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdPrimitiveI8 {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                },
                AnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                    &value.value,
                    &local_acc,
                    &generate_jsonb_set_target("std_vec_vec_std_option_option_std_primitive_i8"),
                    &generate_jsonb_set_path("std_vec_vec_std_option_option_std_primitive_i8"),
                    increment,
                ) {
                    Ok(value) => {
                        local_acc = value;
                    }
                    Err(error) => {
                        return Err(AnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdOptionOptionStdPrimitiveI8 {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                },
                AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                    &value.value,
                    &local_acc,
                    &generate_jsonb_set_target("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"),
                    &generate_jsonb_set_path("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"),
                    increment,
                ) {
                    Ok(value) => {
                        local_acc = value;
                    }
                    Err(error) => {
                        return Err(AnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                },
            }
        }
        Ok(local_acc)
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0 {
            match element {
                AnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
                AnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
                AnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
                AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
                AnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
                AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
            }
        }
        query
    }
}
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
pub struct ObjectWithIdAnimal {
    pub id: postgresql_crud::json_types::UuidOptionToUpdate,
    pub std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8,
    pub std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8,
    pub std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdPrimitiveI8,
    pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8,
    pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
}
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
pub struct ObjectAnimal {
    pub std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8,
    pub std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8,
    pub std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdPrimitiveI8,
    pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8,
    pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
}
pub type ObjectAnimalToCreate = AnimalToCreateWithoutGeneratedId;
pub type ObjectAnimalOptionsToRead = AnimalOptionsToReadWithoutId;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectAnimalFieldReader(std::vec::Vec<AnimalFieldToReadWithoutId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ObjectAnimalFieldReaderTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithoutId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl ObjectAnimalFieldReader {
    pub fn try_new(value: std::vec::Vec<AnimalFieldToReadWithoutId>) -> Result<Self, ObjectAnimalFieldReaderTryNewErrorNamed> {
        if value.is_empty() {
            return Err(ObjectAnimalFieldReaderTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(ObjectAnimalFieldReaderTryNewErrorNamed::NotUniqueFieldFilter {
                    field: element,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            } else {
                unique.push(element);
            }
        }
        Ok(Self(unique))
    }
}
impl<'de> serde::Deserialize<'de> for ObjectAnimalFieldReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<ObjectAnimalFieldReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = ObjectAnimalFieldReader;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct ObjectAnimalFieldReader")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalFieldToReadWithoutId> = <std::vec::Vec<AnimalFieldToReadWithoutId> as serde::Deserialize>::deserialize(__e)?;
                match ObjectAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalFieldToReadWithoutId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct ObjectAnimalFieldReader with 1 element"));
                    }
                };
                match ObjectAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "ObjectAnimalFieldReader",
            __Visitor {
                marker: serde::__private::PhantomData::<ObjectAnimalFieldReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ObjectAnimalFieldReader {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
pub type ObjectAnimalReader = ObjectAnimalOptionsToRead;
pub type ObjectAnimalOptionToUpdate = AnimalOptionToUpdate;
pub type ObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = AnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed;
pub type ObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamedWithSerializeDeserialize = AnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamedWithSerializeDeserialize;
impl postgresql_crud::PostgresqlJsonType for ObjectAnimal {
    type ToCreate<'a> = ObjectAnimalToCreate;
    fn try_generate_postgresql_query_part_to_create(to_create: &Self::ToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        to_create.try_generate_postgresql_query_part_to_create(increment)
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(to_create: Self::ToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        to_create.bind_value_to_postgresql_query_part_to_create(query)
    }
    type FieldReader<'a> = ObjectAnimalFieldReader;
    type OptionsToRead<'a> = ObjectAnimalOptionsToRead;
    fn generate_postgresql_query_part_to_read(field_reader: &Self::FieldReader<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in &field_reader.0 {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithoutId::StdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_primitive_i8", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))")
    }
    type OptionToUpdate<'a> = ObjectAnimalOptionToUpdate;
    type OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = ObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed;
    fn try_generate_postgresql_query_part_to_update(
        option_to_update: &Self::OptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
        option_to_update.try_generate_postgresql_query_part_to_update(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment)
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(option_to_update: Self::OptionToUpdate<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        option_to_update.bind_value_to_postgresql_query_part_to_update(query)
    }
}
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
pub struct StdOptionOptionObjectAnimal(pub std::option::Option<ObjectAnimal>);
pub type StdOptionOptionObjectAnimalToCreateOrigin = AnimalToCreateWithoutGeneratedId;
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
pub struct StdOptionOptionObjectAnimalToCreate(pub std::option::Option<StdOptionOptionObjectAnimalToCreateOrigin>);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionObjectAnimalToCreate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
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
pub struct StdOptionOptionObjectAnimalOptionsToRead(pub std::option::Option<AnimalOptionsToReadWithoutId>);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionObjectAnimalOptionsToRead {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdOptionOptionObjectAnimalFieldReader(std::vec::Vec<AnimalFieldToReadWithoutId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdOptionOptionObjectAnimalFieldReaderTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithoutId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdOptionOptionObjectAnimalFieldReader {
    pub fn try_new(value: std::vec::Vec<AnimalFieldToReadWithoutId>) -> Result<Self, StdOptionOptionObjectAnimalFieldReaderTryNewErrorNamed> {
        if value.is_empty() {
            return Err(StdOptionOptionObjectAnimalFieldReaderTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(StdOptionOptionObjectAnimalFieldReaderTryNewErrorNamed::NotUniqueFieldFilter {
                    field: element,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            } else {
                unique.push(element);
            }
        }
        Ok(Self(unique))
    }
}
impl<'de> serde::Deserialize<'de> for StdOptionOptionObjectAnimalFieldReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<StdOptionOptionObjectAnimalFieldReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = StdOptionOptionObjectAnimalFieldReader;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct StdOptionOptionObjectAnimalFieldReader")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalFieldToReadWithoutId> = <std::vec::Vec<AnimalFieldToReadWithoutId> as serde::Deserialize>::deserialize(__e)?;
                match StdOptionOptionObjectAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalFieldToReadWithoutId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct StdOptionOptionObjectAnimalFieldReader with 1 element"));
                    }
                };
                match StdOptionOptionObjectAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "StdOptionOptionObjectAnimalFieldReader",
            __Visitor {
                marker: serde::__private::PhantomData::<StdOptionOptionObjectAnimalFieldReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionObjectAnimalFieldReader {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
pub type StdOptionOptionObjectAnimalReader = AnimalOptionsToReadWithoutId;
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
pub struct StdOptionOptionObjectAnimalOptionToUpdate(pub std::option::Option<AnimalOptionToUpdate>);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionObjectAnimalOptionToUpdate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl postgresql_crud::PostgresqlJsonType for StdOptionOptionObjectAnimal {
    type ToCreate<'a> = StdOptionOptionObjectAnimalToCreate;
    fn try_generate_postgresql_query_part_to_create(to_create: &Self::ToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        match &to_create.0 {
            Some(value) => match value.try_generate_postgresql_query_part_to_create(increment) {
                Ok(value) => Ok(value),
                Err(error) => Err(error),
            },
            None => Ok(std::string::String::from("null")),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(to_create: Self::ToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let Some(value) = to_create.0 {
            query = value.bind_value_to_postgresql_query_part_to_create(query);
        }
        query
    }
    type FieldReader<'a> = StdOptionOptionObjectAnimalFieldReader;
    type OptionsToRead<'a> = StdOptionOptionObjectAnimalOptionsToRead;
    fn generate_postgresql_query_part_to_read(field_reader: &Self::FieldReader<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in &field_reader.0 {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithoutId::StdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_primitive_i8", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        format!("jsonb_build_object('{field_ident}', case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('value', null) else jsonb_build_object('value',{acc}) end)")
    }
    type OptionToUpdate<'a> = StdOptionOptionObjectAnimalOptionToUpdate;
    type OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = StdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed;
    fn try_generate_postgresql_query_part_to_update(
        option_to_update: &Self::OptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
        Ok(match &option_to_update.0 {
            Some(value) => {
                let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
                let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
                for element in &value.0 {
                    match element {
                        AnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("std_primitive_i8"), "std_primitive_i8", increment) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(StdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        AnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                &value.value,
                                &std_option_option_object_acc,
                                &generate_jsonb_set_target("std_option_option_std_primitive_i8"),
                                "std_option_option_std_primitive_i8",
                                increment,
                            ) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(StdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        AnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                &value.value,
                                &std_option_option_object_acc,
                                &generate_jsonb_set_target("std_vec_vec_std_primitive_i8"),
                                "std_vec_vec_std_primitive_i8",
                                increment,
                            ) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(StdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                &value.value,
                                &std_option_option_object_acc,
                                &generate_jsonb_set_target("std_option_option_std_vec_vec_std_primitive_i8"),
                                "std_option_option_std_vec_vec_std_primitive_i8",
                                increment,
                            ) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(StdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        AnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                &value.value,
                                &std_option_option_object_acc,
                                &generate_jsonb_set_target("std_vec_vec_std_option_option_std_primitive_i8"),
                                "std_vec_vec_std_option_option_std_primitive_i8",
                                increment,
                            ) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(StdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdOptionOptionStdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                &value.value,
                                &std_option_option_object_acc,
                                &generate_jsonb_set_target("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"),
                                "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                                increment,
                            ) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(StdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                    }
                }
                format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',{std_option_option_object_acc})")
            }
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")
                }
                None => {
                    return Err(StdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                }
            },
        })
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(option_to_update: Self::OptionToUpdate<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match option_to_update.0 {
            Some(value) => {
                for element in value.0 {
                    match element {
                        AnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                        AnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                        AnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                        AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                        AnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                        AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                    }
                }
            }
            None => {
                query = query.bind(sqlx::types::Json(None::<std::option::Option<std::vec::Vec<AnimalOptionToUpdateOrigin>>>));
            }
        }
        query
    }
}
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
pub struct StdVecVecObjectWithIdAnimal(std::vec::Vec<ObjectWithIdAnimal>);
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
pub struct StdVecVecObjectWithIdAnimalToCreate(pub std::vec::Vec<AnimalToCreateWithGeneratedId>);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdVecVecObjectWithIdAnimalToCreate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ])
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdVecVecObjectWithIdAnimalOptionsToRead(std::vec::Vec<AnimalOptionsToReadWithId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdVecVecObjectWithIdAnimalOptionsToReadTryNewErrorNamed {
    NotUniqueId {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdVecVecObjectWithIdAnimalOptionsToRead {
    pub fn try_new(value: std::vec::Vec<AnimalOptionsToReadWithId>) -> Result<Self, StdVecVecObjectWithIdAnimalOptionsToReadTryNewErrorNamed> {
        {
            let mut acc = vec![];
            for element in &value {
                if let Some(value) = &element.id {
                    if acc.contains(&&value.value) {
                        return Err(StdVecVecObjectWithIdAnimalOptionsToReadTryNewErrorNamed::NotUniqueId {
                            error: format!("not unique id {}", value.value.0),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    } else {
                        acc.push(&value.value);
                    }
                }
            }
        }
        Ok(Self(value))
    }
}
impl<'de> serde::Deserialize<'de> for StdVecVecObjectWithIdAnimalOptionsToRead {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<StdVecVecObjectWithIdAnimalOptionsToRead>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = StdVecVecObjectWithIdAnimalOptionsToRead;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct StdVecVecObjectWithIdAnimalOptionsToRead")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalOptionsToReadWithId> = <std::vec::Vec<AnimalOptionsToReadWithId> as serde::Deserialize>::deserialize(__e)?;
                match StdVecVecObjectWithIdAnimalOptionsToRead::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalOptionsToReadWithId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct StdVecVecObjectWithIdAnimalOptionsToRead with 1 element"));
                    }
                };
                match StdVecVecObjectWithIdAnimalOptionsToRead::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "StdVecVecObjectWithIdAnimalOptionsToRead",
            __Visitor {
                marker: serde::__private::PhantomData::<StdVecVecObjectWithIdAnimalOptionsToRead>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdVecVecObjectWithIdAnimalOptionsToRead {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ])
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdVecVecObjectWithIdAnimalFieldReader {
    field_vec: std::vec::Vec<AnimalFieldToReadWithId>,
    pagination: postgresql_crud::Pagination,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdVecVecObjectWithIdAnimalFieldReaderTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdVecVecObjectWithIdAnimalFieldReader {
    pub fn try_new(field_vec: std::vec::Vec<AnimalFieldToReadWithId>, pagination: postgresql_crud::Pagination) -> Result<Self, StdVecVecObjectWithIdAnimalFieldReaderTryNewErrorNamed> {
        if field_vec.is_empty() {
            return Err(StdVecVecObjectWithIdAnimalFieldReaderTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in field_vec {
            if unique.contains(&element) {
                return Err(StdVecVecObjectWithIdAnimalFieldReaderTryNewErrorNamed::NotUniqueFieldFilter {
                    field: element,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            } else {
                unique.push(element);
            }
        }
        Ok(Self { field_vec: unique, pagination })
    }
}
impl<'de> serde::Deserialize<'de> for StdVecVecObjectWithIdAnimalFieldReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "field_vec" => serde::__private::Ok(__Field::__field0),
                    "pagination" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"field_vec" => serde::__private::Ok(__Field::__field0),
                    b"pagination" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<StdVecVecObjectWithIdAnimalFieldReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = StdVecVecObjectWithIdAnimalFieldReader;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct StdVecVecObjectWithIdAnimalFieldReader")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalFieldToReadWithId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct StdVecVecObjectWithIdAnimalFieldReader with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<postgresql_crud::Pagination>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct StdVecVecObjectWithIdAnimalFieldReader with 2 elements"));
                    }
                };
                match StdVecVecObjectWithIdAnimalFieldReader::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::vec::Vec<AnimalFieldToReadWithId>> = serde::__private::None;
                let mut __field1: serde::__private::Option<postgresql_crud::Pagination> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_vec"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<AnimalFieldToReadWithId>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("pagination"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<postgresql_crud::Pagination>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("field_vec")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("pagination")?,
                };
                match StdVecVecObjectWithIdAnimalFieldReader::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["field_vec", "pagination"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "StdVecVecObjectWithIdAnimalFieldReader",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<StdVecVecObjectWithIdAnimalFieldReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdVecVecObjectWithIdAnimalFieldReader {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            field_vec: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            pagination: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub type StdVecVecObjectWithIdAnimalReader = StdVecVecObjectWithIdAnimalOptionsToRead;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdVecVecObjectWithIdAnimalJsonArrayChange {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    create: std::vec::Vec<AnimalToCreateWithGeneratedId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    update: std::vec::Vec<AnimalOptionsToUpdate>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    delete: std::vec::Vec<postgresql_crud::json_types::UuidOptionToUpdate>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed {
    CreateUpdateDeleteCheckFieldsAreEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdInJsonUpdateArray {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdInJsonDeleteArray {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdInJsonUpdateAndDeleteArrays {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdVecVecObjectWithIdAnimalJsonArrayChange {
    pub fn try_new(create: std::vec::Vec<AnimalToCreateWithGeneratedId>, update: std::vec::Vec<AnimalOptionsToUpdate>, delete: std::vec::Vec<postgresql_crud::json_types::UuidOptionToUpdate>) -> Result<Self, StdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed> {
        if create.is_empty() && update.is_empty() && delete.is_empty() {
            return Err(StdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed::CreateUpdateDeleteCheckFieldsAreEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let update_acc = {
                let mut update_acc = vec![];
                for element in &update {
                    let id = &element.id;
                    if update_acc.contains(&id) {
                        return Err(StdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed::NotUniqueIdInJsonUpdateArray {
                            error: format!("custom serde error deserializing StdVecVecObjectWithIdAnimalJsonArrayChange: not unique id in json update array: {}", id.0),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    } else {
                        update_acc.push(id);
                    }
                }
                update_acc
            };
            let delete_acc = {
                let mut delete_acc = vec![];
                for element in &delete {
                    if delete_acc.contains(&element) {
                        return Err(StdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed::NotUniqueIdInJsonDeleteArray {
                            error: format!("custom serde error deserializing StdVecVecObjectWithIdAnimalJsonArrayChange: not unique id in json delete array: {}", element.0),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    } else {
                        delete_acc.push(element);
                    }
                }
                delete_acc
            };
            for element in update_acc {
                if delete_acc.contains(&&element) {
                    return Err(StdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed::NotUniqueIdInJsonUpdateAndDeleteArrays {
                        error: format!("custom serde error deserializing StdVecVecObjectWithIdAnimalJsonArrayChange: not unique id in json update and delete arrays: {}", element.0),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { create, update, delete })
    }
}
impl<'de> serde::Deserialize<'de> for StdVecVecObjectWithIdAnimalJsonArrayChange {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "create" => serde::__private::Ok(__Field::__field0),
                    "update" => serde::__private::Ok(__Field::__field1),
                    "delete" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"create" => serde::__private::Ok(__Field::__field0),
                    b"update" => serde::__private::Ok(__Field::__field1),
                    b"delete" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<StdVecVecObjectWithIdAnimalJsonArrayChange>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = StdVecVecObjectWithIdAnimalJsonArrayChange;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct StdVecVecObjectWithIdAnimalJsonArrayChange")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalToCreateWithGeneratedId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalOptionsToUpdate>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::vec::Vec<postgresql_crud::json_types::UuidOptionToUpdate>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                match StdVecVecObjectWithIdAnimalJsonArrayChange::try_new(__field0, __field1, __field2) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::vec::Vec<AnimalToCreateWithGeneratedId>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::vec::Vec<AnimalOptionsToUpdate>> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::vec::Vec<postgresql_crud::json_types::UuidOptionToUpdate>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<AnimalToCreateWithGeneratedId>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<AnimalOptionsToUpdate>>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<postgresql_crud::json_types::UuidOptionToUpdate>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => {
                        vec![]
                    }
                };
                match StdVecVecObjectWithIdAnimalJsonArrayChange::try_new(__field0, __field1, __field2) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["create", "update", "delete"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "StdVecVecObjectWithIdAnimalJsonArrayChange",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<StdVecVecObjectWithIdAnimalJsonArrayChange>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl StdVecVecObjectWithIdAnimalJsonArrayChange {
    fn try_generate_postgresql_query_part_to_update(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed> {
        let update_query_part_acc = {
            let mut element_acc = std::string::String::from("elem");
            if self.update.is_empty() {
                element_acc
            } else {
                let mut update_query_part_acc = std::string::String::default();
                let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
                for element_handle in &self.update {
                    let id_increment = match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            increment.to_string()
                        }
                        None => {
                            return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                        }
                    };
                    for element in &element_handle.fields.0 {
                        match element {
                            AnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(&value.value, &element_acc, &generate_jsonb_set_target("std_primitive_i8"), "std_primitive_i8", increment) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                            AnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                    &value.value,
                                    &element_acc,
                                    &generate_jsonb_set_target("std_option_option_std_primitive_i8"),
                                    "std_option_option_std_primitive_i8",
                                    increment,
                                ) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                            AnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                    &value.value,
                                    &element_acc,
                                    &generate_jsonb_set_target("std_vec_vec_std_primitive_i8"),
                                    "std_vec_vec_std_primitive_i8",
                                    increment,
                                ) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                            AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                    &value.value,
                                    &element_acc,
                                    &generate_jsonb_set_target("std_option_option_std_vec_vec_std_primitive_i8"),
                                    "std_option_option_std_vec_vec_std_primitive_i8",
                                    increment,
                                ) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                            AnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                    &value.value,
                                    &element_acc,
                                    &generate_jsonb_set_target("std_vec_vec_std_option_option_std_primitive_i8"),
                                    "std_vec_vec_std_option_option_std_primitive_i8",
                                    increment,
                                ) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdOptionOptionStdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                            AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                    &value.value,
                                    &element_acc,
                                    &generate_jsonb_set_target("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"),
                                    "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                                    increment,
                                ) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                    update_query_part_acc.push_str(&format!("when elem ->> 'id' = ${id_increment} then {element_acc} "));
                }
                let _ = update_query_part_acc.pop();
                format!("case {update_query_part_acc} else elem end")
            }
        };
        let delete_query_part_acc = {
            let mut delete_query_part_acc = std::string::String::default();
            for _ in &self.delete {
                match increment.checked_add(1) {
                    Some(value) => {
                        *increment = value;
                        let maybe_space_and_space = if delete_query_part_acc.is_empty() { "" } else { " and " };
                        delete_query_part_acc.push_str(&format!("{maybe_space_and_space}elem->>'id' <> ${increment}"));
                    }
                    None => {
                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                    }
                }
            }
            delete_query_part_acc
        };
        let create_query_part_acc = {
            let mut create_query_part_acc = std::string::String::default();
            for element in &self.create {
                match element.try_generate_postgresql_query_part_to_create(increment) {
                    Ok(value) => {
                        create_query_part_acc.push_str(&format!("{value},"));
                    }
                    Err(error) => {
                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::Create {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            let _ = create_query_part_acc.pop();
            create_query_part_acc
        };
        let maybe_where = if delete_query_part_acc.is_empty() { std::string::String::default() } else { format!(" where {delete_query_part_acc}") };
        let maybe_jsonb_build_array = if create_query_part_acc.is_empty() { std::string::String::default() } else { format!(" || jsonb_build_array({create_query_part_acc})") };
        Ok(format!
        ("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}', case when jsonb_array_length({jsonb_set_target}) = 0 then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}), '[]'::jsonb)) end {maybe_jsonb_build_array})"))
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element_handle in self.update {
            query = query.bind(element_handle.id.0.to_string());
            for element in element_handle.fields.0 {
                match element {
                    AnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                    AnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                    AnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                    AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                    AnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                    AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                }
            }
        }
        for element in self.delete {
            query = query.bind(element.0.to_string());
        }
        for element in self.create {
            query = element.bind_value_to_postgresql_query_part_to_create(query);
        }
        query
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdVecVecObjectWithIdAnimalJsonArrayChange {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            create: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()],
            update: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()],
            delete: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()],
        }
    }
}
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
pub struct StdVecVecObjectWithIdAnimalOptionToUpdate(pub StdVecVecObjectWithIdAnimalJsonArrayChange);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdVecVecObjectWithIdAnimalOptionToUpdate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed {
    AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed {
        #[eo_error_occurence]
        error: AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl postgresql_crud::PostgresqlJsonType for StdVecVecObjectWithIdAnimal {
    type ToCreate<'a> = StdVecVecObjectWithIdAnimalToCreate;
    fn try_generate_postgresql_query_part_to_create(to_create: &Self::ToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        let mut acc = std::string::String::default();
        for element in &to_create.0 {
            match element.try_generate_postgresql_query_part_to_create(increment) {
                Ok(value) => {
                    acc.push_str(&format!("{value},"));
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _ = acc.pop();
        Ok(format!("jsonb_build_array({acc})"))
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(to_create: Self::ToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in to_create.0 {
            query = element.bind_value_to_postgresql_query_part_to_create(query);
        }
        query
    }
    type FieldReader<'a> = StdVecVecObjectWithIdAnimalFieldReader;
    type OptionsToRead<'a> = StdVecVecObjectWithIdAnimalOptionsToRead;
    fn generate_postgresql_query_part_to_read(field_reader: &Self::FieldReader<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in &field_reader.field_vec {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithId::Id(value) => <postgresql_crud::json_types::Uuid as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "id", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdPrimitiveI8(value) => <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdOptionOptionStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_option_option_std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdVecVecStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_vec_vec_std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithId::StdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_option_option_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        let start = field_reader.pagination.start();
        let end = field_reader.pagination.end();
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',(select jsonb_agg({acc}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end})))")
    }
    type OptionToUpdate<'a> = StdVecVecObjectWithIdAnimalOptionToUpdate;
    type OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = StdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed;
    fn try_generate_postgresql_query_part_to_update(
        option_to_update: &Self::OptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
        match option_to_update.0.try_generate_postgresql_query_part_to_update(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment) {
            Ok(value) => Ok(value),
            Err(error) => {
                return Err(StdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        }
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(option_to_update: Self::OptionToUpdate<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = option_to_update.0.bind_value_to_postgresql_query_part_to_update(query);
        query
    }
}
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
pub struct StdOptionOptionStdVecVecObjectWithIdAnimal(std::option::Option<std::vec::Vec<ObjectWithIdAnimal>>);
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
pub struct StdOptionOptionStdVecVecObjectWithIdAnimalToCreate(pub std::option::Option<std::vec::Vec<AnimalToCreateWithGeneratedId>>);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdVecVecObjectWithIdAnimalToCreate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ]))
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead(std::option::Option<std::vec::Vec<AnimalOptionsToReadWithId>>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToReadTryNewErrorNamed {
    NotUniqueId {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead {
    pub fn try_new(value: std::option::Option<std::vec::Vec<AnimalOptionsToReadWithId>>) -> Result<Self, StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToReadTryNewErrorNamed> {
        match value {
            Some(value) => {
                let mut acc = vec![];
                for element in &value {
                    if let Some(value) = &element.id {
                        if acc.contains(&&value.value) {
                            return Err(StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToReadTryNewErrorNamed::NotUniqueId {
                                error: format!("not unique id {}", value.value.0),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(&value.value);
                        }
                    }
                }
                Ok(Self(Some(value)))
            }
            None => Ok(Self(None)),
        }
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ]))
    }
}
impl<'de> serde::Deserialize<'de> for StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::option::Option<std::vec::Vec<AnimalOptionsToReadWithId>> = <std::option::Option<std::vec::Vec<AnimalOptionsToReadWithId>> as serde::Deserialize>::deserialize(__e)?;
                match StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<std::vec::Vec<AnimalOptionsToReadWithId>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead with 1 element"));
                    }
                };
                match StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead",
            __Visitor {
                marker: serde::__private::PhantomData::<StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader {
    field_vec: std::vec::Vec<AnimalFieldToReadWithId>,
    pagination: postgresql_crud::Pagination,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdOptionOptionStdVecVecObjectWithIdAnimalFieldReaderTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader {
    pub fn try_new(field_vec: std::vec::Vec<AnimalFieldToReadWithId>, pagination: postgresql_crud::Pagination) -> Result<Self, StdOptionOptionStdVecVecObjectWithIdAnimalFieldReaderTryNewErrorNamed> {
        if field_vec.is_empty() {
            return Err(StdOptionOptionStdVecVecObjectWithIdAnimalFieldReaderTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in field_vec {
            if unique.contains(&element) {
                return Err(StdOptionOptionStdVecVecObjectWithIdAnimalFieldReaderTryNewErrorNamed::NotUniqueFieldFilter {
                    field: element,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            } else {
                unique.push(element);
            }
        }
        Ok(Self { field_vec: unique, pagination })
    }
}
impl<'de> serde::Deserialize<'de> for StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "field_vec" => serde::__private::Ok(__Field::__field0),
                    "pagination" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"field_vec" => serde::__private::Ok(__Field::__field0),
                    b"pagination" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalFieldToReadWithId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<postgresql_crud::Pagination>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader with 2 elements"));
                    }
                };
                match StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::vec::Vec<AnimalFieldToReadWithId>> = serde::__private::None;
                let mut __field1: serde::__private::Option<postgresql_crud::Pagination> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_vec"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<AnimalFieldToReadWithId>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("pagination"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<postgresql_crud::Pagination>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("field_vec")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("pagination")?,
                };
                match StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["field_vec", "pagination"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            field_vec: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            pagination: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub type StdOptionOptionStdVecVecObjectWithIdAnimalReader = StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    create: std::vec::Vec<AnimalToCreateWithGeneratedId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    update: std::vec::Vec<AnimalOptionsToUpdate>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    delete: std::vec::Vec<postgresql_crud::json_types::UuidOptionToUpdate>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed {
    NotUniqueIdInJsonUpdateArray {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdInJsonDeleteArray {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdInJsonUpdateAndDeleteArrays {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange {
    pub fn try_new(create: std::vec::Vec<AnimalToCreateWithGeneratedId>, update: std::vec::Vec<AnimalOptionsToUpdate>, delete: std::vec::Vec<postgresql_crud::json_types::UuidOptionToUpdate>) -> Result<Self, StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed> {
        {
            let update_acc = {
                let mut update_acc = vec![];
                for element in &update {
                    let id = &element.id;
                    if update_acc.contains(&id) {
                        return Err(StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed::NotUniqueIdInJsonUpdateArray {
                            error: format!("custom serde error deserializing StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange: not unique id in json update array: {}", id.0),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    } else {
                        update_acc.push(id);
                    }
                }
                update_acc
            };
            let delete_acc = {
                let mut delete_acc = vec![];
                for element in &delete {
                    if delete_acc.contains(&element) {
                        return Err(StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed::NotUniqueIdInJsonDeleteArray {
                            error: format!("custom serde error deserializing StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange: not unique id in json delete array: {}", element.0),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    } else {
                        delete_acc.push(element);
                    }
                }
                delete_acc
            };
            for element in update_acc {
                if delete_acc.contains(&&element) {
                    return Err(StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChangeTryNewErrorNamed::NotUniqueIdInJsonUpdateAndDeleteArrays {
                        error: format!("custom serde error deserializing StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange: not unique id in json update and delete arrays: {}", element.0),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { create, update, delete })
    }
}
impl<'de> serde::Deserialize<'de> for StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "create" => serde::__private::Ok(__Field::__field0),
                    "update" => serde::__private::Ok(__Field::__field1),
                    "delete" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"create" => serde::__private::Ok(__Field::__field0),
                    b"update" => serde::__private::Ok(__Field::__field1),
                    b"delete" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalToCreateWithGeneratedId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalOptionsToUpdate>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::vec::Vec<postgresql_crud::json_types::UuidOptionToUpdate>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                match StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange::try_new(__field0, __field1, __field2) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::vec::Vec<AnimalToCreateWithGeneratedId>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::vec::Vec<AnimalOptionsToUpdate>> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::vec::Vec<postgresql_crud::json_types::UuidOptionToUpdate>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<AnimalToCreateWithGeneratedId>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<AnimalOptionsToUpdate>>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<postgresql_crud::json_types::UuidOptionToUpdate>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => {
                        vec![]
                    }
                };
                match StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange::try_new(__field0, __field1, __field2) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["create", "update", "delete"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange {
    fn try_generate_postgresql_query_part_to_update(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed> {
        let update_query_part_acc = {
            let mut element_acc = std::string::String::from("elem");
            if self.update.is_empty() {
                element_acc
            } else {
                let mut update_query_part_acc = std::string::String::default();
                let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
                for element_handle in &self.update {
                    let id_increment = match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            increment.to_string()
                        }
                        None => {
                            return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                        }
                    };
                    for element in &element_handle.fields.0 {
                        match element {
                            AnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(&value.value, &element_acc, &generate_jsonb_set_target("std_primitive_i8"), "std_primitive_i8", increment) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                            AnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                    &value.value,
                                    &element_acc,
                                    &generate_jsonb_set_target("std_option_option_std_primitive_i8"),
                                    "std_option_option_std_primitive_i8",
                                    increment,
                                ) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                            AnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                    &value.value,
                                    &element_acc,
                                    &generate_jsonb_set_target("std_vec_vec_std_primitive_i8"),
                                    "std_vec_vec_std_primitive_i8",
                                    increment,
                                ) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                            AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                    &value.value,
                                    &element_acc,
                                    &generate_jsonb_set_target("std_option_option_std_vec_vec_std_primitive_i8"),
                                    "std_option_option_std_vec_vec_std_primitive_i8",
                                    increment,
                                ) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                            AnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                    &value.value,
                                    &element_acc,
                                    &generate_jsonb_set_target("std_vec_vec_std_option_option_std_primitive_i8"),
                                    "std_vec_vec_std_option_option_std_primitive_i8",
                                    increment,
                                ) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdOptionOptionStdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                            AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                                match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                    &value.value,
                                    &element_acc,
                                    &generate_jsonb_set_target("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"),
                                    "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                                    increment,
                                ) {
                                    Ok(value) => {
                                        element_acc = value;
                                    }
                                    Err(error) => {
                                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
                                            error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                    update_query_part_acc.push_str(&format!("when elem ->> 'id' = ${id_increment} then {element_acc} "));
                }
                let _ = update_query_part_acc.pop();
                format!("case {update_query_part_acc} else elem end")
            }
        };
        let delete_query_part_acc = {
            let mut delete_query_part_acc = std::string::String::default();
            for _ in &self.delete {
                match increment.checked_add(1) {
                    Some(value) => {
                        *increment = value;
                        let maybe_space_and_space = if delete_query_part_acc.is_empty() { "" } else { " and " };
                        delete_query_part_acc.push_str(&format!("{maybe_space_and_space}elem->>'id' <> ${increment}"));
                    }
                    None => {
                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                    }
                }
            }
            delete_query_part_acc
        };
        let create_query_part_acc = {
            let mut create_query_part_acc = std::string::String::default();
            for element in &self.create {
                match element.try_generate_postgresql_query_part_to_create(increment) {
                    Ok(value) => {
                        create_query_part_acc.push_str(&format!("{value},"));
                    }
                    Err(error) => {
                        return Err(AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed::Create {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            let _ = create_query_part_acc.pop();
            create_query_part_acc
        };
        let maybe_where = if delete_query_part_acc.is_empty() { std::string::String::default() } else { format!(" where {delete_query_part_acc}") };
        let maybe_jsonb_build_array = if create_query_part_acc.is_empty() { std::string::String::default() } else { format!(" || jsonb_build_array({create_query_part_acc})") };
        Ok(format!
        ("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}', case when jsonb_typeof({jsonb_set_target}) = 'array' then case when jsonb_array_length({jsonb_set_target}) = 0 then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}), '[]'::jsonb)) end else '[]'::jsonb end {maybe_jsonb_build_array})"))
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element_handle in self.update {
            query = query.bind(element_handle.id.0.to_string());
            for element in element_handle.fields.0 {
                match element {
                    AnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                    AnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                    AnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                    AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                    AnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                    AnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                    }
                }
            }
        }
        for element in self.delete {
            query = query.bind(element.0.to_string());
        }
        for element in self.create {
            query = element.bind_value_to_postgresql_query_part_to_create(query);
        }
        query
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            create: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()],
            update: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()],
            delete: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()],
        }
    }
}
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
pub struct StdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdate(pub std::option::Option<StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange>);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdate {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonArrayChange {
        #[eo_error_occurence]
        error: AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl postgresql_crud::PostgresqlJsonType for StdOptionOptionStdVecVecObjectWithIdAnimal {
    type ToCreate<'a> = StdOptionOptionStdVecVecObjectWithIdAnimalToCreate;
    fn try_generate_postgresql_query_part_to_create(to_create: &Self::ToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        match &to_create.0 {
            Some(value) => {
                let mut acc = std::string::String::default();
                for element in value {
                    match element.try_generate_postgresql_query_part_to_create(increment) {
                        Ok(value) => {
                            acc.push_str(&format!("{value},"));
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                let _ = acc.pop();
                Ok(format!("jsonb_build_array({acc})"))
            }
            None => Ok(std::string::String::from("null")),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(to_create: Self::ToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let Some(value) = to_create.0 {
            for element in value {
                query = element.bind_value_to_postgresql_query_part_to_create(query);
            }
        }
        query
    }
    type FieldReader<'a> = StdOptionOptionStdVecVecObjectWithIdAnimalFieldReader;
    type OptionsToRead<'a> = StdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead;
    fn generate_postgresql_query_part_to_read(field_reader: &Self::FieldReader<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in &field_reader.field_vec {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithId::Id(value) => <postgresql_crud::json_types::Uuid as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "id", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdPrimitiveI8(value) => <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdOptionOptionStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_option_option_std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdVecVecStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_vec_vec_std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithId::StdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_option_option_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        let start = field_reader.pagination.start();
        let end = field_reader.pagination.end();
        format!
        ("jsonb_build_object('{field_ident}', jsonb_build_object('value', case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then null else (select jsonb_agg({acc}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end}) end))")
    }
    type OptionToUpdate<'a> = StdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdate;
    type OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = StdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed;
    fn try_generate_postgresql_query_part_to_update(
        option_to_update: &Self::OptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
        match &option_to_update.0 {
            Some(value) => match value.try_generate_postgresql_query_part_to_update(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment) {
                Ok(value) => Ok(value),
                Err(error) => {
                    return Err(StdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::JsonArrayChange {
                        error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            },
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                }
                None => {
                    return Err(StdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                }
            },
        }
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(option_to_update: Self::OptionToUpdate<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match option_to_update.0 {
            Some(value) => {
                query = value.bind_value_to_postgresql_query_part_to_update(query);
            }
            None => {
                query = query.bind(sqlx::types::Json(None::<std::option::Option<StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange>>));
            }
        }
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesJsonObjectAnimal(pub sqlx::types::Json<ObjectAnimal>);
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdOptionOptionSqlxTypesJsonObjectAnimal(pub std::option::Option<sqlx::types::Json<ObjectAnimal>>);
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesJsonStdOptionOptionObjectAnimal(pub sqlx::types::Json<StdOptionOptionObjectAnimal>);
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdOptionOptionSqlxTypesJsonStdOptionOptionObjectAnimal(pub std::option::Option<sqlx::types::Json<StdOptionOptionObjectAnimal>>);
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesJsonStdVecVecObjectWithIdAnimal(pub sqlx::types::Json<StdVecVecObjectWithIdAnimal>);
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdOptionOptionSqlxTypesJsonStdVecVecObjectWithIdAnimal(pub std::option::Option<sqlx::types::Json<StdVecVecObjectWithIdAnimal>>);
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesJsonStdOptionOptionStdVecVecObjectWithIdAnimal(pub sqlx::types::Json<StdOptionOptionStdVecVecObjectWithIdAnimal>);
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdOptionOptionSqlxTypesJsonStdOptionOptionStdVecVecObjectWithIdAnimal(pub std::option::Option<sqlx::types::Json<StdOptionOptionStdVecVecObjectWithIdAnimal>>);
