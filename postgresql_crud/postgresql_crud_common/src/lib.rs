pub mod generate_postgresql_json_type;
pub mod pagination;
pub mod postgresql_json_type;
pub mod postgresql_type;
pub mod value;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum QueryPartErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
//todo add another error variant instead for CreateQueryPartErrorNamed
impl std::convert::From<crate::postgresql_json_type::postgresql_json_type_trait::CreateQueryPartErrorNamed> for QueryPartErrorNamed {
    fn from(value: crate::postgresql_json_type::postgresql_json_type_trait::CreateQueryPartErrorNamed) -> Self {
        match value {
            crate::postgresql_json_type::postgresql_json_type_trait::CreateQueryPartErrorNamed::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
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
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for LogicalOperator {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
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
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for Order {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
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

pub trait BindQuery<'a> {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, QueryPartErrorNamed>;
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}
