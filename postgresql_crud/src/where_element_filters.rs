generate_where_element_filters::generate_where_element_filters!();


// #[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema, serde :: Deserialize)]
// pub struct PostgresqlJsonTypeWhereElementEqual<T> {
//     pub logical_operator: crate::LogicalOperator,
//     value: T,
// }
// impl<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementEqual<T> {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self {
//             logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//             value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//         }
//     }
// }
// impl<'a, T: std::marker::Send + serde::Serialize + 'a> crate::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementEqual<T> {
//     fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
//         match increment.checked_add(1) {
//             Some(value) => {
//                 *increment = value;
//                 Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
//             }
//             None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(sqlx::types::Json(self.value));
//         query
//     }
// }