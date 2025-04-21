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




#[derive(Debug)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin(std::vec::Vec<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin>);
impl VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    pub fn new(value: std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>) -> Self {
        Self(value.into_iter().map(|element| OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin::new(element)).collect())
    }
}
impl std::fmt::Display for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<std::primitive::i8> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<std::primitive::i8> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self.0.clone()), buf)
    }
}
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect {
    dimension1_pagination: crate::Pagination,
    dimension2_pagination: crate::Pagination,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimension2_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
    Equal(crate::where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
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
impl error_occurence_lib::ToStdStringString for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
impl crate::PostgresqlJsonType for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber {
    type Create = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
    type Select = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        let dimension2_start = value.dimension2_pagination.start();
        let dimension2_end = value.dimension2_pagination.end();
        format!("
        
        
jsonb_build_object(
  '{field_ident}', 
  jsonb_build_object(
    'value', 
    (
      select 
        jsonb_agg(
          (
            case when jsonb_typeof(outer_elem.value) = 'array' then (
                select 
                  jsonb_agg(inner_elem.value) 
                from 
                  jsonb_array_elements(outer_elem.value) with ordinality as inner_elem(value, inner_ord) 
                where 
                  inner_ord between {dimension2_start} 
                  and {dimension2_end}
            )
            else 
            null
            end
          )
        ) 
      from 
        jsonb_array_elements(
          {column_name_and_maybe_field_getter} -> '{field_ident}'
        ) with ordinality as outer_elem(value, outer_ord) 
      where 
        outer_ord between {dimension1_start} 
        and {dimension1_end}
    )
  )
)

                            
            
            
        ")
    }
    type WhereElement = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement;
    type Read = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin;
    type Update = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
