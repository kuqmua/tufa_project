pub mod postgresql_json_type;
pub mod postgresql_type;
pub mod where_element_filters;

pub use futures::TryStreamExt;
pub use http_logic;
pub use http_logic::GetAxumHttpStatusCode;
pub use route_validators::check_body_size;
pub use route_validators::check_commit;
pub use strum_macros::EnumIter;
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

pub use generate_postgresql_crud::GeneratePostgresqlCrud;
pub use generate_postgresql_json_type::GeneratePostgresqlJsonType;
pub use generate_postgresql_json_types::generate_postgresql_json_types;
pub use generate_postgresql_types::generate_postgresql_types;

pub use naming::CommitSnakeCase;
pub use naming::CommitUpperCamelCase;

pub trait PostgresqlType {
    type PostgresqlTypeSelf: std::fmt::Debug;
    type Create: std::fmt::Debug + Clone + PartialEq + serde::Serialize + for<'__> serde::Deserialize<'__> + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed>;
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>;
    type Select: std::fmt::Debug + Clone + PartialEq + serde::Serialize + for<'__> serde::Deserialize<'__> + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String;
    type WhereElement: std::fmt::Debug + Clone + PartialEq + serde::Serialize + for<'__> serde::Deserialize<'__> + for<'a> crate::PostgresqlTypeWhereFilter<'a>;
    type Read: std::fmt::Debug + Clone + PartialEq + serde::Serialize + for<'__> serde::Deserialize<'__> + for<'__> sqlx::Decode<'__, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>;
    type Update: std::fmt::Debug + Clone + PartialEq + serde::Serialize + for<'__> serde::Deserialize<'__> + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed>;
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

pub trait PostgresqlTypeWhereFilter<'a> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed>;
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

pub trait PostgresqlTypePrimaryKey {
    type PrimaryKey;
}

pub trait PostgresqlJsonType {
    type Create<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    fn create_query_part(value: &Self::Create<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed>;
    fn create_query_bind<'a>(value: Self::Create<'a>, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
    type Select<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    fn select_query_part(
        value: &Self::Select<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        //todo remove this coz its used properly now
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
        is_postgresql_type: std::primitive::bool,
    ) -> std::string::String;
    type WhereElement<'a>: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        // + schemars::JsonSchema //todo
        + crate::PostgresqlTypeWhereFilter<'a>
        + crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    type Read<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    type Update<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    fn update_query_part(value: &Self::Update<'_>, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed>;
    fn update_query_bind<'a>(value: Self::Update<'_>, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

pub trait GeneratePostgresqlJsonTypeToRead {
    fn generate_postgresql_json_type_to_read_from_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String
    where
        Self: Sized;
}

pub fn wrap_into_jsonb_build_object(field: &std::primitive::str, value: &std::primitive::str) -> std::string::String {
    format!("jsonb_build_object('{field}',{value})||")
}
pub fn maybe_primary_key(is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
    if is_primary_key { "primary key" } else { "" }
}

pub trait CombinationOfTraitsForPostgresqlCrudLogic: app_state::GetSourcePlaceType + app_state::GetTimezone + app_state::GetPostgresPool + Send + Sync {}

pub trait DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement: Sized {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self;
}
pub trait AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement: Sized {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self>;
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum QueryPartErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct Pagination {
    limit: std::primitive::i64,
    offset: std::primitive::i64,
}
impl<'de> serde::Deserialize<'de> for Pagination {
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
                    "limit" => serde::__private::Ok(__Field::__field0),
                    "offset" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"limit" => serde::__private::Ok(__Field::__field0),
                    b"offset" => serde::__private::Ok(__Field::__field1),
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
            marker: serde::__private::PhantomData<Pagination>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = Pagination;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct Pagination")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct Pagination with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct Pagination with 2 elements"));
                    }
                };
                match Pagination::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::primitive::i64> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::primitive::i64> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("limit"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("offset"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("limit")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("offset")?,
                };
                match Pagination::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["limit", "offset"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "Pagination",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<Pagination>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum PaginationTryNewErrorNamed {
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::i64,
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    LimitIsLessThanOrEqualToZero {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OffsetIsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Pagination {
    pub fn try_new(limit: std::primitive::i64, offset: std::primitive::i64) -> Result<Self, PaginationTryNewErrorNamed> {
        if limit <= 0 || offset < 0 {
            if limit <= 0 {
                Err(PaginationTryNewErrorNamed::LimitIsLessThanOrEqualToZero {
                    limit,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            } else {
                Err(PaginationTryNewErrorNamed::OffsetIsLessThanZero {
                    offset,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            }
        } else {
            if offset.checked_add(limit).is_some() {
                Ok(Self { limit, offset })
            } else {
                Err(PaginationTryNewErrorNamed::OffsetPlusLimitIsIntOverflow {
                    limit,
                    offset,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            }
        }
    }
    pub fn start(&self) -> std::primitive::i64 {
        self.offset
    }
    pub fn end(&self) -> std::primitive::i64 {
        self.offset + self.limit
    }
}
//for Read in GeneratePostgresqlCrud
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for Pagination {
    fn query_part(&self, increment: &mut std::primitive::u64, _: &dyn std::fmt::Display, _: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(limit_increment) => {
                *increment = limit_increment;
                match increment.checked_add(1) {
                    Some(offset_increment) => {
                        *increment = offset_increment;
                        Ok(format!("limit ${limit_increment} offset ${offset_increment}"))
                    }
                    None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.limit);
        query = query.bind(self.offset);
        query
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for Pagination {
    #[inline]
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { limit: 3, offset: std::default::Default::default() }
    }
}

//this needed coz serde std::option::Option<std::option::Option<T>> #[serde(skip_serializing_if = "Option::is_none")] - if both options: inner and parent is null then it skip - its not correct
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct Value<T> {
    pub value: T,
}

//todo ExactSizeIterator now is not a solution. error[E0658]: use of unstable library feature `exact_size_is_empty`. maybe rewrite it later
pub trait IsEmpty {
    fn is_empty(&self) -> std::primitive::bool;
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PostgresqlTypeWhere<PostgresqlTypeWhereElement> {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<PostgresqlTypeWhereElement>,
}
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
)]
pub enum PostgresqlTypeWhereTryNewErrorNamed<PostgresqlTypeWhereElement> {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: PostgresqlTypeWhereElement,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl<PostgresqlTypeWhereElement: std::cmp::PartialEq + Clone> PostgresqlTypeWhere<PostgresqlTypeWhereElement> {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<PostgresqlTypeWhereElement>) -> Result<Self, PostgresqlTypeWhereTryNewErrorNamed<PostgresqlTypeWhereElement>> {
        if value.is_empty() {
            return Err(PostgresqlTypeWhereTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlTypeWhereTryNewErrorNamed::NotUnique {
                        value: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { logical_operator, value })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, PostgresqlTypeWhereElement: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + _serde::Deserialize<'de>> _serde::Deserialize<'de> for PostgresqlTypeWhere<PostgresqlTypeWhereElement> {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
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
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de, PostgresqlTypeWhere> {
                marker: _serde::__private::PhantomData<PostgresqlTypeWhere>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de, PostgresqlTypeWhereElement: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + _serde::Deserialize<'de>> _serde::de::Visitor<'de> for __Visitor<'de, PostgresqlTypeWhereElement> {
                type Value = PostgresqlTypeWhere<PostgresqlTypeWhereElement>;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeWhere")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeWhere with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<PostgresqlTypeWhereElement>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeWhere with 2 elements"));
                        }
                    };
                    match PostgresqlTypeWhere::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::vec::Vec<PostgresqlTypeWhereElement>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<PostgresqlTypeWhereElement>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match PostgresqlTypeWhere::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeWhere",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlTypeWhereElement>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl<'a, PostgresqlTypeWhereElement: crate::PostgresqlTypeWhereFilter<'a>> crate::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhere<PostgresqlTypeWhereElement> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for element in &self.value {
            match crate::PostgresqlTypeWhereFilter::query_part(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
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
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.value {
            query = crate::PostgresqlTypeWhereFilter::query_bind(element, query);
        }
        query
    }
}
impl<PostgresqlTypeWhereElement: crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhere<PostgresqlTypeWhereElement> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}

///////////////////////
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
// )]
// pub struct ObjectUpdate<T>(std::vec::Vec<T>);
// #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum ObjectUpdateTryNewErrorNamed {
//     IsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUnique {
//         #[eo_to_std_string_string_serialize_deserialize]
//         error: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl ObjectUpdate {
//     pub fn try_new(value: std::vec::Vec<ObjectUpdateOrigin>) -> Result<Self, ObjectUpdateTryNewErrorNamed> {
//         if value.is_empty() {
//             return Err(ObjectUpdateTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//         }
//         {
//             let mut acc = vec![];
//             let generate_not_unique_field = |value: &std::primitive::str| format!("not unique {value} field");
//             for element in &value {
//                 match element {
//                     ObjectUpdateOrigin::StdPrimitiveI8(_) => {
//                         let value = ObjectFieldToUpdate::StdPrimitiveI8;
//                         if acc.contains(&value) {
//                             return Err(ObjectUpdateTryNewErrorNamed::NotUnique {
//                                 error: generate_not_unique_field("std_primitive_i8"),
//                                 code_occurence: error_occurence_lib::code_occurence!(),
//                             });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                 }
//             }
//         }
//         Ok(Self(value))
//     }
// }
// impl<'de> serde::Deserialize<'de> for ObjectUpdate {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<ObjectUpdate>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = ObjectUpdate;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "tuple struct ObjectUpdate")
//             }
//             #[inline]
//             fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
//             where
//                 __E: serde::Deserializer<'de>,
//             {
//                 let __field0: std::vec::Vec<ObjectUpdateOrigin> = <std::vec::Vec<ObjectUpdateOrigin> as serde::Deserialize>::deserialize(__e)?;
//                 match ObjectUpdate::try_new(__field0) {
//                     Ok(value) => serde::__private::Ok(value),
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(format!("{error:?}")));
//                     }
//                 }
//             }
//             #[inline]
//             fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<ObjectUpdateOrigin>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct ObjectUpdate with 1 element"));
//                     }
//                 };
//                 match ObjectUpdate::try_new(__field0) {
//                     Ok(value) => serde::__private::Ok(value),
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(format!("{error:?}")));
//                     }
//                 }
//             }
//         }
//         serde::Deserializer::deserialize_newtype_struct(
//             __deserializer,
//             "ObjectUpdate",
//             __Visitor {
//                 marker: serde::__private::PhantomData::<ObjectUpdate>,
//                 lifetime: serde::__private::PhantomData,
//             },
//         )
//     }
// }
// impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectUpdate {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
// impl ObjectUpdate {
//     fn update_query_part(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
//         let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
//         let generate_jsonb_set_path = |value: &std::primitive::str| {
//             let previous = match jsonb_set_path.is_empty() {
//                 true => std::string::String::default(),
//                 false => format!("{jsonb_set_path},"),
//             };
//             format!("{previous}{value}")
//         };
//         let mut local_acc = format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end)");
//         for element in &self.0 {
//             match &element {
//                 ObjectUpdateOrigin::StdPrimitiveI8(value) => {
//                     match <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &local_acc, &generate_jsonb_set_target("std_primitive_i8"), &generate_jsonb_set_path("std_primitive_i8"), increment) {
//                         Ok(value) => {
//                             local_acc = value;
//                         }
//                         Err(error) => {
//                             return Err(error);
//                         }
//                     }
//                 }
//             }
//         }
//         Ok(local_acc)
//     }
//     fn update_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         for element in self.0 {
//             match element {
//                 ObjectUpdateOrigin::StdPrimitiveI8(value) => {
//                     query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
//                 }
//             }
//         }
//         query
//     }
// }