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
        "not_null_or_nullable": "NotNull",
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
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "Nullable"
            }
        }
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": {
            "ArrayDimension2": {
                "dimension1_not_null_or_nullable": "NotNull",
                "dimension2_not_null_or_nullable": "NotNull"
            }
        }
    },
    // {
    //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
    //     "not_null_or_nullable": "NotNull",
    //     "postgresql_json_type_pattern": {
    //         "ArrayDimension2": {
    //             "dimension1_not_null_or_nullable": "NotNull",
    //             "dimension2_not_null_or_nullable": "Nullable"
    //         }
    //     }
    // },
    // {
    //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
    //     "not_null_or_nullable": "NotNull",
    //     "postgresql_json_type_pattern": {
    //         "ArrayDimension2": {
    //             "dimension1_not_null_or_nullable": "Nullable",
    //             "dimension2_not_null_or_nullable": "NotNull"
    //         }
    //     }
    // },
    // {
    //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
    //     "not_null_or_nullable": "NotNull",
    //     "postgresql_json_type_pattern": {
    //         "ArrayDimension2": {
    //             "dimension1_not_null_or_nullable": "Nullable",
    //             "dimension2_not_null_or_nullable": "Nullable"
    //         }
    //     }
    // },
    // {
    //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
    //     "not_null_or_nullable": "Nullable",
    //     "postgresql_json_type_pattern": {
    //         "ArrayDimension2": {
    //             "dimension1_not_null_or_nullable": "NotNull",
    //             "dimension2_not_null_or_nullable": "NotNull"
    //         }
    //     }
    // },
    // {
    //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
    //     "not_null_or_nullable": "Nullable",
    //     "postgresql_json_type_pattern": {
    //         "ArrayDimension2": {
    //             "dimension1_not_null_or_nullable": "NotNull",
    //             "dimension2_not_null_or_nullable": "Nullable"
    //         }
    //     }
    // },
    // {
    //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
    //     "not_null_or_nullable": "Nullable",
    //     "postgresql_json_type_pattern": {
    //         "ArrayDimension2": {
    //             "dimension1_not_null_or_nullable": "Nullable",
    //             "dimension2_not_null_or_nullable": "NotNull"
    //         }
    //     }
    // },
    // {
    //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
    //     "not_null_or_nullable": "Nullable",
    //     "postgresql_json_type_pattern": {
    //         "ArrayDimension2": {
    //             "dimension1_not_null_or_nullable": "Nullable",
    //             "dimension2_not_null_or_nullable": "Nullable"
    //         }
    //     }
    // },
    {
        "postgresql_json_type": "UuidUuidAsJsonbString",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    }
]);




// #[derive(Debug)]
// pub struct VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber;
// #[derive(Debug, Clone, PartialEq, PartialOrd, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin(std::vec::Vec<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin>);
// impl VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
//     pub fn new(value: std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>) -> Self {
//         Self(value.into_iter().map(|element| VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin::new(element)).collect())
//     }
// }
// impl std::fmt::Display for VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{self:?}")
//     }
// }
// impl error_occurence_lib::ToStdStringString for VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <sqlx::types::Json<std::primitive::i8> as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <sqlx::types::Json<std::primitive::i8> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self.0.clone()), buf)
//     }
// }
// #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect {
//     dimension1_pagination: crate::Pagination,
//     dimension2_pagination: crate::Pagination,
//     dimension3_pagination: crate::Pagination,
// }
// impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self {
//             dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//             dimension2_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//             dimension3_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
// pub enum VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
//     Equal(crate::where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin>),
// }
// impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
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
// impl error_occurence_lib::ToStdStringString for VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         vec![Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
//     }
// }
// impl crate::PostgresqlJsonType for VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber {
//     type Create = VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin;
//     fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
//     type Select = VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect;
//     fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, no_need_to_wrap: std::primitive::bool) -> std::string::String {
//         let dimension1_start = value.dimension1_pagination.start();
//         let dimension1_end = value.dimension1_pagination.end();
//         let dimension2_start = value.dimension2_pagination.start();
//         let dimension2_end = value.dimension2_pagination.end();
//         let dimension3_start = value.dimension3_pagination.start();
//         let dimension3_end = value.dimension3_pagination.end();
//         format!("
        
// jsonb_build_object(
//   '{field_ident}', 
//   jsonb_build_object(
//     'value', 
//     (
//       select jsonb_agg(
//           (
        
        
//           select 
//             jsonb_agg(
//               case when jsonb_typeof(d2_elem.value)= 'array' then (
//                 select 
//                   jsonb_agg(d3_elem.value) 
//                 from 
//                   jsonb_array_elements(d2_elem.value) with ordinality as d3_elem(value, d3_ord) 
//                 where 
//                   d3_ord between {dimension3_start} 
//                   and {dimension3_end}
//               ) else null end
//             ) 
//           from 
//             jsonb_array_elements(d1_elem.value) with ordinality as d2_elem(value, d2_ord) 
//           where 
//             d2_ord between {dimension2_start} 
//             and {dimension2_end}


//           )
//       ) 
//       from 
//         jsonb_array_elements(
//           {column_name_and_maybe_field_getter} -> '{field_ident}'
//         ) with ordinality as d1_elem(value, d1_ord) 
//       where 
//         d1_ord between {dimension1_start} 
//         and {dimension1_end}
//     )
//   )
// )

        
//         ")
//     }
//     type WhereElement = VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement;
//     type Read = VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin;
//     type Update = VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin;
//     fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
