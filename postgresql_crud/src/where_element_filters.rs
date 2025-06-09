generate_where_element_filters::generate_where_element_filters!();


#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArrayD<
    T: 
    //here
    std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone
> {
    pub logical_operator: crate::LogicalOperator,
    value: 
    crate::NotEmptyUniqueStructVec<T>,
}
impl<T: 
    //here
    std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone
    + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
> crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArrayD<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: 
            crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T:
    std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone 
    + std::marker::Send + serde::Serialize + 'a
> crate::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArrayD<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            } 
        };
        Ok(format!("{}({} @> {value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.value.query_bind(query);
        query
    }
}







// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
// pub struct PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
//     pub logical_operator: crate::LogicalOperator,
//     value: crate::NotEmptyUniqueStructVec<T>,
// }
// impl<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T> {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self {
//             logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//             value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//         }
//     }
// }
// impl<'a, T: std::marker::Send + serde::Serialize + 'a> crate::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T> {
//     fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
//         let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
//             Ok(value) => value,
//             Err(error) => {
//                 return Err(error);
//             }
//         };
//         Ok(format!("{}({} @> {value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column))
//     }
//     fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = self.value.query_bind(query);
//         query
//     }
// }