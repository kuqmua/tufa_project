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
        let ident = naming::parameter::PostgresqlTypeWhereElementSelfUpperCamelCase::from_display(&filter);
        let is_fields_are_public = true;
        let struct_token_stream = {
            let maybe_pub_token_stream = if is_fields_are_public {
                quote::quote! {pub}
            }
            else {
                proc_macro2::TokenStream::new()
            };
            let maybe_serde_deserialize_token_stream = match &filter {
                Filter::Equal => quote::quote!{, serde::Deserialize},
                Filter::GreaterThan => quote::quote!{},
                Filter::Between => quote::quote!{},
                Filter::In => quote::quote!{},
                Filter::CaseSensitiveRegularExpression => quote::quote!{},
                Filter::CaseInsensitiveRegularExpression => quote::quote!{},
                Filter::Before => quote::quote!{},
                Filter::CurrentDate => quote::quote!{},
                Filter::GreaterThanCurrentDate => quote::quote!{},
                Filter::CurrentTimestamp => quote::quote!{},
                Filter::GreaterThanCurrentTimestamp => quote::quote!{},
                Filter::CurrentTime => quote::quote!{},
                Filter::GreaterThanCurrentTime => quote::quote!{},
                Filter::LengthEqual => quote::quote!{},
                Filter::LengthMoreThan => quote::quote!{},
                Filter::EqualToEncodedStringRepresentation => quote::quote!{},
                Filter::ValueIsContainedWithinRange => quote::quote!{},
                Filter::ContainsAnotherRange => quote::quote!{},
                Filter::StrictlyToLeftOfRange => quote::quote!{},
                Filter::StrictlyToRightOfRange => quote::quote!{},
                Filter::IncludedLowerBound => quote::quote!{},
                Filter::ExcludedUpperBound => quote::quote!{},
                Filter::GreaterThanLowerBound => quote::quote!{},
                Filter::OverlapWithRange => quote::quote!{},
                Filter::AdjacentWithRange => quote::quote!{},
                Filter::RangeLength => quote::quote!{},
                // Filter:://BitVecPositionEqual => quote::quote!{},
                Filter::PositionEqual => quote::quote!{},
                Filter::PositionGreaterThan => quote::quote!{},
                Filter::PositionCaseSensitiveRegularExpression => quote::quote!{},
                Filter::PositionCaseInsensitiveRegularExpression => quote::quote!{},
                Filter::ContainsAllElementsOfArray => quote::quote!{},
                Filter::ContainedInArray => quote::quote!{},
                Filter::OverlapsWithArray => quote::quote!{},
                Filter::AllElementsEqual => quote::quote!{},
                Filter::ContainsElementGreaterThan => quote::quote!{},
                Filter::AllElementsGreaterThan => quote::quote!{},
                Filter::ContainsElementCaseSensitiveRegularExpression => quote::quote!{},
                Filter::ContainsElementCaseInsensitiveRegularExpression => quote::quote!{},
                Filter::AllElementsCaseSensitiveRegularExpression => quote::quote!{},
                Filter::AllElementsCaseInsensitiveRegularExpression => quote::quote!{},
                Filter::EqualSecondDimension => quote::quote!{},
            };
            let content_token_stream = match &filter {
                Filter::Equal => quote::quote!{pub value: T},
                Filter::GreaterThan => quote::quote!{},
                Filter::Between => quote::quote!{},
                Filter::In => quote::quote!{},
                Filter::CaseSensitiveRegularExpression => quote::quote!{},
                Filter::CaseInsensitiveRegularExpression => quote::quote!{},
                Filter::Before => quote::quote!{},
                Filter::CurrentDate => quote::quote!{},
                Filter::GreaterThanCurrentDate => quote::quote!{},
                Filter::CurrentTimestamp => quote::quote!{},
                Filter::GreaterThanCurrentTimestamp => quote::quote!{},
                Filter::CurrentTime => quote::quote!{},
                Filter::GreaterThanCurrentTime => quote::quote!{},
                Filter::LengthEqual => quote::quote!{},
                Filter::LengthMoreThan => quote::quote!{},
                Filter::EqualToEncodedStringRepresentation => quote::quote!{},
                Filter::ValueIsContainedWithinRange => quote::quote!{},
                Filter::ContainsAnotherRange => quote::quote!{},
                Filter::StrictlyToLeftOfRange => quote::quote!{},
                Filter::StrictlyToRightOfRange => quote::quote!{},
                Filter::IncludedLowerBound => quote::quote!{},
                Filter::ExcludedUpperBound => quote::quote!{},
                Filter::GreaterThanLowerBound => quote::quote!{},
                Filter::OverlapWithRange => quote::quote!{},
                Filter::AdjacentWithRange => quote::quote!{},
                Filter::RangeLength => quote::quote!{},
                // Filter:://BitVecPositionEqual => quote::quote!{},
                Filter::PositionEqual => quote::quote!{},
                Filter::PositionGreaterThan => quote::quote!{},
                Filter::PositionCaseSensitiveRegularExpression => quote::quote!{},
                Filter::PositionCaseInsensitiveRegularExpression => quote::quote!{},
                Filter::ContainsAllElementsOfArray => quote::quote!{},
                Filter::ContainedInArray => quote::quote!{},
                Filter::OverlapsWithArray => quote::quote!{},
                Filter::AllElementsEqual => quote::quote!{},
                Filter::ContainsElementGreaterThan => quote::quote!{},
                Filter::AllElementsGreaterThan => quote::quote!{},
                Filter::ContainsElementCaseSensitiveRegularExpression => quote::quote!{},
                Filter::ContainsElementCaseInsensitiveRegularExpression => quote::quote!{},
                Filter::AllElementsCaseSensitiveRegularExpression => quote::quote!{},
                Filter::AllElementsCaseInsensitiveRegularExpression => quote::quote!{},
                Filter::EqualSecondDimension => quote::quote!{},
            };
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize #maybe_serde_deserialize_token_stream )]
                pub struct #ident<T> {
                    #maybe_pub_token_stream logical_operator: crate::LogicalOperator,
                    #content_token_stream
                }
            }
        };
        let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = postgresql_crud_macros_common::generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
            &quote::quote!{<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>},
            &postgresql_crud_macros_common::PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::Crate,
            &ident,
            &quote::quote!{<T>},
            &{
                let content_token_stream = match &filter {
                    Filter::Equal => quote::quote!{value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()},
                    Filter::GreaterThan => quote::quote!{},
                    Filter::Between => quote::quote!{},
                    Filter::In => quote::quote!{},
                    Filter::CaseSensitiveRegularExpression => quote::quote!{},
                    Filter::CaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::Before => quote::quote!{},
                    Filter::CurrentDate => quote::quote!{},
                    Filter::GreaterThanCurrentDate => quote::quote!{},
                    Filter::CurrentTimestamp => quote::quote!{},
                    Filter::GreaterThanCurrentTimestamp => quote::quote!{},
                    Filter::CurrentTime => quote::quote!{},
                    Filter::GreaterThanCurrentTime => quote::quote!{},
                    Filter::LengthEqual => quote::quote!{},
                    Filter::LengthMoreThan => quote::quote!{},
                    Filter::EqualToEncodedStringRepresentation => quote::quote!{},
                    Filter::ValueIsContainedWithinRange => quote::quote!{},
                    Filter::ContainsAnotherRange => quote::quote!{},
                    Filter::StrictlyToLeftOfRange => quote::quote!{},
                    Filter::StrictlyToRightOfRange => quote::quote!{},
                    Filter::IncludedLowerBound => quote::quote!{},
                    Filter::ExcludedUpperBound => quote::quote!{},
                    Filter::GreaterThanLowerBound => quote::quote!{},
                    Filter::OverlapWithRange => quote::quote!{},
                    Filter::AdjacentWithRange => quote::quote!{},
                    Filter::RangeLength => quote::quote!{},
                    // Filter:://BitVecPositionEqual => quote::quote!{},
                    Filter::PositionEqual => quote::quote!{},
                    Filter::PositionGreaterThan => quote::quote!{},
                    Filter::PositionCaseSensitiveRegularExpression => quote::quote!{},
                    Filter::PositionCaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::ContainsAllElementsOfArray => quote::quote!{},
                    Filter::ContainedInArray => quote::quote!{},
                    Filter::OverlapsWithArray => quote::quote!{},
                    Filter::AllElementsEqual => quote::quote!{},
                    Filter::ContainsElementGreaterThan => quote::quote!{},
                    Filter::AllElementsGreaterThan => quote::quote!{},
                    Filter::ContainsElementCaseSensitiveRegularExpression => quote::quote!{},
                    Filter::ContainsElementCaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::AllElementsCaseSensitiveRegularExpression => quote::quote!{},
                    Filter::AllElementsCaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::EqualSecondDimension => quote::quote!{},
                };
                quote::quote!{
                    Self {
                        logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                        #content_token_stream
                    }
                }
            },
        );
        let impl_postgresql_type_self_where_filter_token_stream = {
            let where_query_part_token_stream = {
                match &filter {
                    Filter::Equal => quote::quote!{
                        match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
                            }
                            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                        }
                    },
                    Filter::GreaterThan => quote::quote!{},
                    Filter::Between => quote::quote!{},
                    Filter::In => quote::quote!{},
                    Filter::CaseSensitiveRegularExpression => quote::quote!{},
                    Filter::CaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::Before => quote::quote!{},
                    Filter::CurrentDate => quote::quote!{},
                    Filter::GreaterThanCurrentDate => quote::quote!{},
                    Filter::CurrentTimestamp => quote::quote!{},
                    Filter::GreaterThanCurrentTimestamp => quote::quote!{},
                    Filter::CurrentTime => quote::quote!{},
                    Filter::GreaterThanCurrentTime => quote::quote!{},
                    Filter::LengthEqual => quote::quote!{},
                    Filter::LengthMoreThan => quote::quote!{},
                    Filter::EqualToEncodedStringRepresentation => quote::quote!{},
                    Filter::ValueIsContainedWithinRange => quote::quote!{},
                    Filter::ContainsAnotherRange => quote::quote!{},
                    Filter::StrictlyToLeftOfRange => quote::quote!{},
                    Filter::StrictlyToRightOfRange => quote::quote!{},
                    Filter::IncludedLowerBound => quote::quote!{},
                    Filter::ExcludedUpperBound => quote::quote!{},
                    Filter::GreaterThanLowerBound => quote::quote!{},
                    Filter::OverlapWithRange => quote::quote!{},
                    Filter::AdjacentWithRange => quote::quote!{},
                    Filter::RangeLength => quote::quote!{},
                    // Filter:://BitVecPositionEqual => quote::quote!{},
                    Filter::PositionEqual => quote::quote!{},
                    Filter::PositionGreaterThan => quote::quote!{},
                    Filter::PositionCaseSensitiveRegularExpression => quote::quote!{},
                    Filter::PositionCaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::ContainsAllElementsOfArray => quote::quote!{},
                    Filter::ContainedInArray => quote::quote!{},
                    Filter::OverlapsWithArray => quote::quote!{},
                    Filter::AllElementsEqual => quote::quote!{},
                    Filter::ContainsElementGreaterThan => quote::quote!{},
                    Filter::AllElementsGreaterThan => quote::quote!{},
                    Filter::ContainsElementCaseSensitiveRegularExpression => quote::quote!{},
                    Filter::ContainsElementCaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::AllElementsCaseSensitiveRegularExpression => quote::quote!{},
                    Filter::AllElementsCaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::EqualSecondDimension => quote::quote!{},
                }
            };
            let where_query_bind_token_stream = {
                match &filter {
                    Filter::Equal => quote::quote!{
                        query = query.bind(self.value);
                        query
                    },
                    Filter::GreaterThan => quote::quote!{},
                    Filter::Between => quote::quote!{},
                    Filter::In => quote::quote!{},
                    Filter::CaseSensitiveRegularExpression => quote::quote!{},
                    Filter::CaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::Before => quote::quote!{},
                    Filter::CurrentDate => quote::quote!{},
                    Filter::GreaterThanCurrentDate => quote::quote!{},
                    Filter::CurrentTimestamp => quote::quote!{},
                    Filter::GreaterThanCurrentTimestamp => quote::quote!{},
                    Filter::CurrentTime => quote::quote!{},
                    Filter::GreaterThanCurrentTime => quote::quote!{},
                    Filter::LengthEqual => quote::quote!{},
                    Filter::LengthMoreThan => quote::quote!{},
                    Filter::EqualToEncodedStringRepresentation => quote::quote!{},
                    Filter::ValueIsContainedWithinRange => quote::quote!{},
                    Filter::ContainsAnotherRange => quote::quote!{},
                    Filter::StrictlyToLeftOfRange => quote::quote!{},
                    Filter::StrictlyToRightOfRange => quote::quote!{},
                    Filter::IncludedLowerBound => quote::quote!{},
                    Filter::ExcludedUpperBound => quote::quote!{},
                    Filter::GreaterThanLowerBound => quote::quote!{},
                    Filter::OverlapWithRange => quote::quote!{},
                    Filter::AdjacentWithRange => quote::quote!{},
                    Filter::RangeLength => quote::quote!{},
                    // Filter:://BitVecPositionEqual => quote::quote!{},
                    Filter::PositionEqual => quote::quote!{},
                    Filter::PositionGreaterThan => quote::quote!{},
                    Filter::PositionCaseSensitiveRegularExpression => quote::quote!{},
                    Filter::PositionCaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::ContainsAllElementsOfArray => quote::quote!{},
                    Filter::ContainedInArray => quote::quote!{},
                    Filter::OverlapsWithArray => quote::quote!{},
                    Filter::AllElementsEqual => quote::quote!{},
                    Filter::ContainsElementGreaterThan => quote::quote!{},
                    Filter::AllElementsGreaterThan => quote::quote!{},
                    Filter::ContainsElementCaseSensitiveRegularExpression => quote::quote!{},
                    Filter::ContainsElementCaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::AllElementsCaseSensitiveRegularExpression => quote::quote!{},
                    Filter::AllElementsCaseInsensitiveRegularExpression => quote::quote!{},
                    Filter::EqualSecondDimension => quote::quote!{},
                }
            };
            quote::quote! {
                impl<'a, T: sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + 'a + std::marker::Send> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for #ident<T> {
                    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
                        #where_query_part_token_stream
                    }
                    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        #where_query_bind_token_stream
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