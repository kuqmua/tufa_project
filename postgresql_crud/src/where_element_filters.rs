generate_where_element_filters::generate_where_element_filters!();


#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementCaseSensitiveRegularExpressionF {
    pub logical_operator: crate::LogicalOperator,
    regular_expression_case: crate::RegularExpressionCase,
    value: crate::RegexRegex,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementCaseSensitiveRegularExpressionF {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementCaseSensitiveRegularExpressionF {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(trim(both '\"' from ({})::text) {} ${})",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    self.regular_expression_case.postgreql_syntax(),
                    increment
                ))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value.to_string());
        query
    }
}