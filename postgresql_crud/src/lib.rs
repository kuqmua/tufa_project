pub use futures::TryStreamExt;
pub use strum_macros::EnumIter;
pub use uuid::Uuid;
pub use route_validators::check_body_size;
pub use route_validators::check_commit;

pub use generate_postgresql_crud::common_additional_error_variants;
pub use generate_postgresql_crud::create_many_additional_error_variants;
pub use generate_postgresql_crud::create_one_additional_error_variants;
pub use generate_postgresql_crud::delete_many_additional_error_variants;
pub use generate_postgresql_crud::delete_one_additional_error_variants;
pub use generate_postgresql_crud::read_many_additional_error_variants;
pub use generate_postgresql_crud::read_one_additional_error_variants;
pub use generate_postgresql_crud::update_many_additional_error_variants;
pub use generate_postgresql_crud::update_one_additional_error_variants;

pub use generate_postgresql_crud::common_additional_logic;
pub use generate_postgresql_crud::create_many_additional_logic;
pub use generate_postgresql_crud::create_one_additional_logic;
pub use generate_postgresql_crud::delete_many_additional_logic;
pub use generate_postgresql_crud::delete_one_additional_logic;
pub use generate_postgresql_crud::read_many_additional_logic;
pub use generate_postgresql_crud::read_one_additional_logic;
pub use generate_postgresql_crud::update_many_additional_logic;
pub use generate_postgresql_crud::update_one_additional_logic;

pub use generate_postgresql_crud::GeneratePostgresqlCrud;
pub use postgresql_json_object_type::GeneratePostgresqlJsonObjectType;
pub use postgresql_json_object_type::postgresql_json_object_type_pattern;

pub use postgresql_types::*;
pub use postgresql_json_types::*;
pub use where_element_filters::*;
pub use postgresql_crud_common::*;//todo

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct ObjectWithIdAsNotNullJsonbObjectWithIdUpdateElement<T: postgresql_crud_common::PostgresqlJsonType> {
    pub id: <postgresql_json_types::UuidUuidAsNotNullJsonbString as postgresql_crud_common::PostgresqlJsonType>::Update,
    pub fields: <T as postgresql_crud_common::PostgresqlJsonType>::Update,
}
impl<T: postgresql_crud_common::PostgresqlJsonType> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectWithIdAsNotNullJsonbObjectWithIdUpdateElement<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            fields: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<T: postgresql_crud_common::PostgresqlJsonType> ObjectWithIdAsNotNullJsonbObjectWithIdUpdateElement<T> {
    pub fn update_query_part(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let id_increment = match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                increment.to_string()
            }
            None => {
                return Err(postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        };
        match <T as postgresql_crud_common::PostgresqlJsonType>::update_query_part(&self.fields, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment) {
            Ok(value) => Ok(format!("when {jsonb_set_target}->>'id' = ${id_increment} then {value} ")),
            Err(error) => Err(error),
        }
    }
    pub fn update_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.id.query_bind_as_postgresql_text(query);
        query = <T as postgresql_crud_common::PostgresqlJsonType>::update_query_bind(self.fields, query);
        query
    }
}