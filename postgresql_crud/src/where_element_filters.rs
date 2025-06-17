generate_where_element_filters::generate_where_element_filters!();


#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourEqual<T> {
    pub logical_operator: crate::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    // pub dimension1_position: crate::UnsignedPartOfStdPrimitiveI32,
    // pub dimension2_position: crate::UnsignedPartOfStdPrimitiveI32,
    // pub dimension3_position: crate::UnsignedPartOfStdPrimitiveI32,
    // pub dimension4_position: crate::UnsignedPartOfStdPrimitiveI32,
    pub value: T,
}
impl<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            // dimension1_position: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            // dimension2_position: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            // dimension3_position: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            // dimension4_position: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> crate::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        // let increment1 = match increment.checked_add(1) {
        //     Some(value) => {
        //         *increment = value;
        //         value
        //     }
        //     None => {
        //         return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
        //     }
        // };
        // let increment2 = match increment.checked_add(1) {
        //     Some(value) => {
        //         *increment = value;
        //         value
        //     }
        //     None => {
        //         return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
        //     }
        // };
        // let increment3 = match increment.checked_add(1) {
        //     Some(value) => {
        //         *increment = value;
        //         value
        //     }
        //     None => {
        //         return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
        //     }
        // };
        // let increment4 = match increment.checked_add(1) {
        //     Some(value) => {
        //         *increment = value;
        //         value
        //     }
        //     None => {
        //         return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
        //     }
        // };
        // let increment5 = match increment.checked_add(1) {
        //     Some(value) => {
        //         *increment = value;
        //         value
        //     }
        //     None => {
        //         return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
        //     }
        // };
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                value
            }
            None => {
                return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        };
        Ok(format!(
            "{}({}{} = ${})",
            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
            column,
            dimensions_indexes,
            // increment1,
            // increment2,
            // increment3,
            // increment4,
            // increment5,
            value
        ))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        // query = query.bind(self.dimension1_position);
        // query = query.bind(self.dimension2_position);
        // query = query.bind(self.dimension3_position);
        // query = query.bind(self.dimension4_position);
        query = self.dimensions.query_bind(query);
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
