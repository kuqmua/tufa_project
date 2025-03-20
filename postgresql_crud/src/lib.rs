pub use futures::TryStreamExt;
pub use http_logic;
pub use route_validators::check_body_size;
pub use route_validators::check_commit;
pub use uuid::Uuid;

pub use generate_postgresql_crud::common_additional_error_variants;
pub use generate_postgresql_crud::create_many_additional_error_variants;
pub use generate_postgresql_crud::create_one_additional_error_variants;
pub use generate_postgresql_crud::delete_many_additional_error_variants;
pub use generate_postgresql_crud::delete_one_additional_error_variants;
pub use generate_postgresql_crud::read_many_additional_error_variants;
pub use generate_postgresql_crud::read_one_additional_error_variants;
pub use generate_postgresql_crud::update_many_additional_error_variants;
pub use generate_postgresql_crud::update_one_additional_error_variants;

pub use generate_postgresql_crud::common_additional_route_logic;
pub use generate_postgresql_crud::create_many_additional_route_logic;
pub use generate_postgresql_crud::create_one_additional_route_logic;
pub use generate_postgresql_crud::delete_many_additional_route_logic;
pub use generate_postgresql_crud::delete_one_additional_route_logic;
pub use generate_postgresql_crud::read_many_additional_route_logic;
pub use generate_postgresql_crud::read_one_additional_route_logic;
pub use generate_postgresql_crud::update_many_additional_route_logic;
pub use generate_postgresql_crud::update_one_additional_route_logic;
//////////////////////////////
pub use postgresql_crud_common::BindQuery;
pub use postgresql_crud_common::Order;
pub use postgresql_crud_common::OrderBy;
pub use postgresql_crud_common::QueryPartErrorNamed;
pub use postgresql_crud_common::QueryPartErrorNamedWithSerializeDeserialize;
pub use postgresql_crud_common::value::Value;
// pub use postgresql_crud_common::StdVecVecStdPrimitiveU8;
//
pub use http_logic::GetAxumHttpStatusCode;
pub use strum_macros::EnumIter;

//todo move and reexport traits
pub trait CombinationOfTraitsForPostgresqlCrudLogic: app_state::GetSourcePlaceType + app_state::GetTimezone + app_state::GetPostgresPool + Send + Sync {}

pub use naming::CommitSnakeCase;
pub use naming::CommitUpperCamelCase;

pub use generate_postgresql_json_type::GeneratePostgresqlJsonType;

pub use postgresql_crud_common::GeneratePostgresqlJsonTypeToRead;
pub use postgresql_crud_common::generate_postgresql_json_type::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
pub use postgresql_crud_common::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
pub use postgresql_crud_common::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonType;
pub use postgresql_crud_common::postgresql_json_type::postgresql_json_type_trait::CreateQueryPartErrorNamed;
pub use postgresql_crud_common::postgresql_json_type::postgresql_json_type_trait::CreateQueryPartErrorNamedWithSerializeDeserialize;

// pub use postgresql_crud_common::generate_postgresql_json_type::CheckIdExistsInJsonGeneric;
// pub use postgresql_crud_common::generate_postgresql_json_type::CheckIdExistsInJsonStdOptionOptionGeneric;

// pub use postgresql_crud_common::generate_postgresql_json_type::CheckIdExistsInJsonStdOptionOptionStdVecVecGenericWithId;
// pub use postgresql_crud_common::generate_postgresql_json_type::CheckIdExistsInJsonStdVecVecGenericWithId;

// pub use postgresql_crud_common::generate_postgresql_json_type::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed;
// pub use postgresql_crud_common::generate_postgresql_json_type::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamedWithSerializeDeserialize;
// pub use postgresql_crud_common::generate_postgresql_json_type::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed;
// pub use postgresql_crud_common::generate_postgresql_json_type::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamedWithSerializeDeserialize;

// pub use postgresql_crud_common::generate_postgresql_json_type::JsonArrayElementBindQuery;

// pub use postgresql_crud_common::generate_postgresql_json_type::JsonArrayElementUpdateBindQuery;
// pub use postgresql_crud_common::generate_postgresql_json_type::JsonArrayElementDeleteBindQuery;
// pub use postgresql_crud_common::generate_postgresql_json_type::JsonArrayElementCreateBindQuery;

pub use postgresql_crud_common::pagination::Pagination;

pub use postgresql_crud_common::generate_postgresql_json_type::wrap_into_jsonb_build_object;

pub use postgresql_crud_common::LogicalOperator;

pub use postgresql_crud_common::postgresql_json_type::postgresql_json_type;

////////////
pub use generate_postgresql_crud::GeneratePostgresqlCrud;
pub use generate_postgresql_types::generate_postgresql_types;

pub use postgresql_crud_common::postgresql_type;

pub use postgresql_crud_common::maybe_primary_key;

pub use crate::postgresql_type::postgresql_type::PostgresqlTypeWhere;