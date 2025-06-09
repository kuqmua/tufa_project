generate_where_element_filters::generate_where_element_filters!();

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
// pub struct PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
//     pub logical_operator: crate::LogicalOperator,
//     dimension1_position: crate::UnsignedPartOfStdPrimitiveI32,
//     value: crate::NotEmptyUniqueEnumVec<T>,
// }
// impl<
//     T: std::fmt::Debug 
//     + std::cmp::PartialEq
//     + std::clone::Clone
//     + crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
// > crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T> {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self {
//             logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//             dimension1_position: ::core::default::Default::default(),
//             value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//         }
//     }
// }
// impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + std::marker::Send + serde::Serialize + 'a> crate::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T> {
//     fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
//         let increment1 = match increment.checked_add(1) {
//             Some(value) => {
//                 *increment = value;
//                 value
//             }
//             None => {
//                 return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         };
//         let increment2 = match increment.checked_add(1) {
//             Some(value) => {
//                 *increment = value;
//                 value
//             }
//             None => {
//                 return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         };
//         Ok(format!("{}({}->${} @> ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment1, increment2))
//     }
//     fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(self.dimension1_position);
//         query = query.bind(sqlx::types::Json(self.value));
//         query
//     }
// }


#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T> {
    pub logical_operator: crate::LogicalOperator,
    value: 
    // crate::NotEmptyUniqueEnumVec<T>,
    std::vec::Vec<T>,
}
impl<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: 
            
            vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()],
        }
    }
}
impl<'a, T: std::marker::Send + serde::Serialize + 'a> crate::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} @> ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}