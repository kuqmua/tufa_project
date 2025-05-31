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
    },
    // {
    //     "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
    //     "not_null_or_nullable": "NotNull",
    //     "postgresql_json_type_pattern": {
    //         "ArrayDimension1": {
    //             "dimension1_not_null_or_nullable": "NotNull"
    //         }
    //     }
    // },
    {
        "postgresql_json_type": "UuidUuidAsJsonbString",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    }
]);

/////////////
// impl std::convert::TryFrom<StdStringStringAsNotNullJsonbStringOrigin> for UuidUuidAsNotNullJsonbStringOrigin {
//     type Error = uuid::Error;
//     fn try_from(value: StdStringStringAsNotNullJsonbStringOrigin) -> Result<Self, Self::Error> {
//         match uuid::Uuid::parse_str(&value.0) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
/////////////
#[derive(Debug)]
pub struct VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin(std::vec::Vec<StdPrimitiveI16AsNotNullJsonbNumberOrigin>);
impl VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
    pub fn new(value: std::vec::Vec<std::primitive::i16>) -> Self {
        Self(value.into_iter().map(|element| StdPrimitiveI16AsNotNullJsonbNumberOrigin::new(element)).collect())
    }
}
impl std::fmt::Display for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<std::vec::Vec<std::primitive::i16>> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<std::vec::Vec<std::primitive::i16>> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self.0.clone()), buf)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberSelect {
    dimension1_pagination: crate::Pagination,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberWhereElement {
    Equal(crate::where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn new(value: std::vec::Vec<std::primitive::i16>) -> Self {
        Self(VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
pub type VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberReadInner = std::vec::Vec<std::primitive::i16>;
impl crate::PostgresqlJsonType for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumber {
    type TableTypeDeclaration = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin;
    type Create = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin;
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
    type Select = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg((value)) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {dimension1_start} and {dimension1_end})))")
    }
    type WhereElement = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberWhereElement;
    type Read = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberRead;
    type ReadInner = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0 .0.into_iter().map(|element| element.0).collect()
    }
    type Update = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin;
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
/////////////////
//////////////////
/////////////////
#[derive(Debug)]
pub struct VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin(std::vec::Vec<StdPrimitiveI16AsNotNullJsonbNumberOrigin>);
//here
#[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberTryNewErrorNamed {
    LengthIsNotTwo {
        #[eo_to_std_string_string_serialize_deserialize]
        length: std::primitive::u64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
    //here instead of new
    pub fn try_new(value: std::vec::Vec<std::primitive::i16>) -> Result<Self, VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberTryNewErrorNamed> {
        let len = value.len();
        if value.len() == 2 {
            Ok(Self(value.into_iter().map(|element| StdPrimitiveI16AsNotNullJsonbNumberOrigin::new(element)).collect()))
        }
        else {
            Err(VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberTryNewErrorNamed::LengthIsNotTwo {
                length: len as std::primitive::u64,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
        
    }
}
// impl std::fmt::Display for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{self:?}")
//     }
// }
// impl error_occurence_lib::ToStdStringString for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <sqlx::types::Json<std::vec::Vec<std::primitive::i16>> as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <sqlx::types::Json<std::vec::Vec<std::primitive::i16>> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl sqlx::Encode<'_, sqlx::Postgres> for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self.0.clone()), buf)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberSelect {
//     dimension1_pagination: crate::Pagination,
// }
// impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberSelect {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self {
//             dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
// pub enum VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberWhereElement {
//     Equal(crate::where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin>),
// }
// impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberWhereElement {
//     fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
//         match &self {
//             Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//         }
//     }
//     fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         match self {
//             Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
//         }
//     }
// }
// impl error_occurence_lib::ToStdStringString for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberWhereElement {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberWhereElement {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         vec![Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberRead(VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin);
// impl VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberRead {
//     pub fn new(value: std::vec::Vec<std::primitive::i16>) -> Self {
//         Self(VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
//     }
// }
// impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberRead {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
//     }
// }
// pub type VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberReadInner = std::vec::Vec<std::primitive::i16>;
// impl crate::PostgresqlJsonType for VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumber {
//     type TableTypeDeclaration = VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin;
//     type Create = VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin;
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
//     type Select = VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberSelect;
//     fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
//         let dimension1_start = value.dimension1_pagination.start();
//         let dimension1_end = value.dimension1_pagination.end();
//         format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg((value)) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {dimension1_start} and {dimension1_end})))")
//     }
//     type WhereElement = VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberWhereElement;
//     type Read = VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberRead;
//     type ReadInner = VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberReadInner;
//     fn into_inner(value: Self::Read) -> Self::ReadInner {
//         value.0 .0.into_iter().map(|element| element.0).collect()
//     }
//     type Update = VecOfCustomStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin;
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



