#[proc_macro]
pub fn generate_postgresql_type_where_element_filters(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    // struct Properties<'a> {
    //     maybe_pub_token_stream: &'a dyn quote::ToTokens,
    //     maybe_serde_deserialize_token_stream: &'a dyn quote::ToTokens,
    //     struct_additional_fields_token_stream: &'a dyn quote::ToTokens,
    //     impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream: &'a dyn quote::ToTokens,
    //     where_query_part_content_token_stream: &'a dyn quote::ToTokens,
    //     where_query_bind_content_token_stream: &'a dyn quote::ToTokens,
    // }
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
        let (
            maybe_pub_token_stream,
            maybe_serde_deserialize_token_stream,
            struct_additional_fields_token_stream,
            impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream,
            where_query_part_content_token_stream,
            where_query_bind_content_token_stream,
        ) = match &filter {
            Filter::Equal => (
                &quote::quote!{pub},
                &quote::quote!{, serde::Deserialize},
                &quote::quote!{pub value: T},
                &quote::quote!{
                    value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                },
                &quote::quote!{
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
                        }
                        None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                    }
                },
                &quote::quote!{
                    query = query.bind(self.value);
                    query
                },
            ),
            Filter::GreaterThan => todo!(),
            Filter::Between => todo!(),
            Filter::In => todo!(),
            Filter::CaseSensitiveRegularExpression => todo!(),
            Filter::CaseInsensitiveRegularExpression => todo!(),
            Filter::Before => todo!(),
            Filter::CurrentDate => todo!(),
            Filter::GreaterThanCurrentDate => todo!(),
            Filter::CurrentTimestamp => todo!(),
            Filter::GreaterThanCurrentTimestamp => todo!(),
            Filter::CurrentTime => todo!(),
            Filter::GreaterThanCurrentTime => todo!(),
            Filter::LengthEqual => todo!(),
            Filter::LengthMoreThan => todo!(),
            Filter::EqualToEncodedStringRepresentation => todo!(),
            Filter::ValueIsContainedWithinRange => todo!(),
            Filter::ContainsAnotherRange => todo!(),
            Filter::StrictlyToLeftOfRange => todo!(),
            Filter::StrictlyToRightOfRange => todo!(),
            Filter::IncludedLowerBound => todo!(),
            Filter::ExcludedUpperBound => todo!(),
            Filter::GreaterThanLowerBound => todo!(),
            Filter::OverlapWithRange => todo!(),
            Filter::AdjacentWithRange => todo!(),
            Filter::RangeLength => todo!(),
            // Filter:://BitVecPositionEqual => todo!(),
            Filter::PositionEqual => todo!(),
            Filter::PositionGreaterThan => todo!(),
            Filter::PositionCaseSensitiveRegularExpression => todo!(),
            Filter::PositionCaseInsensitiveRegularExpression => todo!(),
            Filter::ContainsAllElementsOfArray => todo!(),
            Filter::ContainedInArray => todo!(),
            Filter::OverlapsWithArray => todo!(),
            Filter::AllElementsEqual => todo!(),
            Filter::ContainsElementGreaterThan => todo!(),
            Filter::AllElementsGreaterThan => todo!(),
            Filter::ContainsElementCaseSensitiveRegularExpression => todo!(),
            Filter::ContainsElementCaseInsensitiveRegularExpression => todo!(),
            Filter::AllElementsCaseSensitiveRegularExpression => todo!(),
            Filter::AllElementsCaseInsensitiveRegularExpression => todo!(),
            Filter::EqualSecondDimension => todo!(),
        };
        let ident = naming::parameter::PostgresqlTypeWhereElementSelfUpperCamelCase::from_display(&filter);
        let struct_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize #maybe_serde_deserialize_token_stream )]
                pub struct #ident<T> {
                    #maybe_pub_token_stream logical_operator: crate::LogicalOperator,
                    #struct_additional_fields_token_stream
                }
            }
        };
        let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = postgresql_crud_macros_common::generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
            &quote::quote!{<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>},
            &postgresql_crud_macros_common::PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::Crate,
            &ident,
            &quote::quote!{<T>},
            &{
                quote::quote!{
                    Self {
                        logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                        #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream
                    }
                }
            },
        );
        let impl_postgresql_type_self_where_filter_token_stream = {
            quote::quote! {
                impl<'a, T: sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + 'a + std::marker::Send> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for #ident<T> {
                    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
                        #where_query_part_content_token_stream
                    }
                    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        #where_query_bind_content_token_stream
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