#[proc_macro]
pub fn generate_postgresql_type_where_element_filters(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    #[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    enum Filter {
        Equal,
        GreaterThan,
        Between,
        In,
        CaseSensitiveRegularExpression,
        CaseInsensitiveRegularExpression,
        Before,
        CurrentDate,
        GreaterThanCurrentDate,
        CurrentTimestamp,
        GreaterThanCurrentTimestamp,
        CurrentTime,
        GreaterThanCurrentTime,
        LengthEqual,
        LengthMoreThan,
        EqualToEncodedStringRepresentation,
        ValueIsContainedWithinRange,
        ContainsAnotherRange,
        StrictlyToLeftOfRange,
        StrictlyToRightOfRange,
        IncludedLowerBound,
        ExcludedUpperBound,
        GreaterThanLowerBound,
        OverlapWithRange,
        AdjacentWithRange,
        RangeLength,
        //BitVecPositionEqual,
        PositionEqual,
        PositionGreaterThan,
        PositionCaseSensitiveRegularExpression,
        PositionCaseInsensitiveRegularExpression,
        ContainsAllElementsOfArray,
        ContainedInArray,
        OverlapsWithArray,
        AllElementsEqual,
        ContainsElementGreaterThan,
        AllElementsGreaterThan,
        ContainsElementCaseSensitiveRegularExpression,
        ContainsElementCaseInsensitiveRegularExpression,
        AllElementsCaseSensitiveRegularExpression,
        AllElementsCaseInsensitiveRegularExpression,
        EqualSecondDimension,
    }
    let generate_filters_token_stream = |filter: &Filter|{
        let struct_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                pub struct PostgresqlTypeWhereElementEqual<T> {
                    pub logical_operator: crate::LogicalOperator,
                    pub value: T,
                }
            }
        };
        let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = {
            quote::quote! {
                impl<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementEqual<T> {
                    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
                        Self {
                            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                        }
                    }
                }
            }
        };
        let impl_postgresql_type_self_where_filter_token_stream = {
            quote::quote! {
                impl<'a, T: sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + 'a + std::marker::Send> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for PostgresqlTypeWhereElementEqual<T> {
                    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
                        match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
                            }
                            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                        }
                    }
                    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        query = query.bind(self.value);
                        query
                    }
                }
            }
        };
        quote::quote! {
            #struct_token_stream
            #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
            #impl_postgresql_type_self_where_filter_token_stream
        }
    };
    // let filter_array_token_stream = Filter::into_array().map(|element|generate_filters_token_stream(&element));
    let equal_token_stream = generate_filters_token_stream(&Filter::Equal);
    let generated = quote::quote! {
        // #(#filter_array_token_stream)*
        #equal_token_stream
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlTypeWhereElementFilters",
    //     &generated,
    // );
    generated.into()
}