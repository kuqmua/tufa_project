pub mod pagination;
pub mod value;
pub mod where_element_filters;
pub mod postgresql_json_type;
pub mod postgresql_json_type_trait;
pub mod postgresql_type;
pub mod postgresql_type_trait;

pub use futures::TryStreamExt;
pub use http_logic;
pub use route_validators::check_body_size;
pub use route_validators::check_commit;
pub use uuid::Uuid;
pub use http_logic::GetAxumHttpStatusCode;
pub use strum_macros::EnumIter;

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

pub use postgresql_json_type_trait::CreateQueryPartErrorNamed;
pub use postgresql_json_type_trait::CreateQueryPartErrorNamedWithSerializeDeserialize;
pub use postgresql_json_type_trait::PostgresqlJsonType;
pub use postgresql_type_trait::PostgresqlTypeSelfWhereFilter;
pub use postgresql_type_trait::PostgresqlType;
pub use postgresql_type::PostgresqlTypeWhere;

pub use naming::CommitSnakeCase;
pub use naming::CommitUpperCamelCase;

pub use generate_postgresql_json_type::GeneratePostgresqlJsonType;

pub use value::Value;
pub use pagination::Pagination;

pub use generate_postgresql_crud::GeneratePostgresqlCrud;
pub use generate_postgresql_types::generate_postgresql_types;
pub use generate_postgresql_json_types::generate_postgresql_json_types;

pub trait CombinationOfTraitsForPostgresqlCrudLogic: app_state::GetSourcePlaceType + app_state::GetTimezone + app_state::GetPostgresPool + Send + Sync {}

pub trait DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement: Sized {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self;
}
pub trait AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement: Sized {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self>;
}

pub fn wrap_into_jsonb_build_object(field: &std::primitive::str, value: &std::primitive::str) -> std::string::String {
    format!("jsonb_build_object('{field}',{value})||")
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum QueryPartErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
//todo add another error variant instead for CreateQueryPartErrorNamed
impl std::convert::From<crate::postgresql_json_type_trait::CreateQueryPartErrorNamed> for QueryPartErrorNamed {
    fn from(value: crate::postgresql_json_type_trait::CreateQueryPartErrorNamed) -> Self {
        match value {
            crate::postgresql_json_type_trait::CreateQueryPartErrorNamed::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Eq, PartialEq, schemars::JsonSchema)]
pub enum LogicalOperator {
    And,
    Or,
    AndNot,
    OrNot,
}
impl LogicalOperator {
    pub fn to_query_part(&self, is_need_to_add_logical_operator: std::primitive::bool) -> std::string::String {
        let not_space = format!("{} ", naming::NotSnakeCase);
        if is_need_to_add_logical_operator {
            let and_space = format!("{} ", naming::AndSnakeCase);
            let or_space = format!("{} ", naming::OrSnakeCase);
            match &self {
                Self::And => and_space,
                Self::Or => or_space,
                Self::AndNot => format!("{and_space}{not_space}"),
                Self::OrNot => format!("{or_space}{not_space}"),
            }
        } else {
            match &self {
                Self::And | Self::Or => std::string::String::default(),
                Self::AndNot | Self::OrNot => not_space,
            }
        }
    }
}
impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl Default for LogicalOperator {
    fn default() -> Self {
        Self::Or
    }
}
impl DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for LogicalOperator {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, from_str::FromStr)]
pub enum Order {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}
impl std::fmt::Display for Order {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(formatter, "{}", naming::AscUpperCamelCase),
            Self::Desc => write!(formatter, "{}", naming::DescUpperCamelCase),
        }
    }
}
impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}
impl DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for Order {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
impl Order {
    pub fn to_upper_camel_case_stringified(&self) -> std::string::String {
        naming::DisplayToUpperCamelCaseStringified::case(&self)
    }
    pub fn to_snake_case_stringified(&self) -> std::string::String {
        naming::DisplayToSnakeCaseStringified::case(&self)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderBy<ColumnGeneric> {
    pub column: ColumnGeneric,
    pub order: Option<Order>,
}

pub trait GeneratePostgresqlJsonTypeToRead {
    fn generate_postgresql_json_type_to_read_from_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String
    where
        Self: Sized;
}

pub fn maybe_primary_key(is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
    if is_primary_key { "primary key" } else { "" }
}

pub trait BindQuery {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, QueryPartErrorNamed>;
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>;
}
