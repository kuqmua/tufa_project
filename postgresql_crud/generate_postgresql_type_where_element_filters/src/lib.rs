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
        let ident_try_new_error_named = naming::parameter::PostgresqlTypeWhereElementSelfTryNewErrorNamedUpperCamelCase::from_display(&filter);
        let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
        let path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = quote::quote!{
            crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
        };

        let pub_snake_case_token_stream = {
            let pub_snake_case = naming::PubSnakeCase;
            quote::quote!{#pub_snake_case}
        };
        let comma_serde_deserialize_token_stream = quote::quote!{, serde::Deserialize};
        let pub_value_t_token_stream = quote::quote!{pub value: T};
        let generate_enum_ident_try_new_error_named_token_stream = |content_token_stream: &dyn quote::ToTokens|{
            quote::quote!{
                #[derive(
                    Debug,
                    Clone,
                    serde::Serialize,
                    serde::Deserialize,
                    thiserror::Error,
                )]
                pub enum #ident_try_new_error_named<T> {
                    #content_token_stream
                }
            }
        };
        let value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = quote::quote!{
            value: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
        };
        let generate_where_query_part_one_value_token_stream = |format_handle_token_stream: &dyn quote::ToTokens|{
            quote::quote!{
                match increment.checked_add(1) {
                    Some(value) => {
                        *increment = value;
                        Ok(format!(#format_handle_token_stream, &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
                    }
                    None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
        };
        let where_query_bind_one_value_token_stream = quote::quote!{
            query = query.bind(self.value);
            query
        };
        let (
            maybe_pub_token_stream,
            maybe_serde_deserialize_token_stream,
            struct_additional_fields_token_stream,
            enum_postgresql_type_where_element_filter_try_new_error_named_token_stream,
            impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream,
            where_query_part_content_token_stream,
            where_query_bind_content_token_stream,
        ) = match &filter {
            Filter::Equal => (
                &pub_snake_case_token_stream,
                &comma_serde_deserialize_token_stream,
                &pub_value_t_token_stream,
                &proc_macro2_token_stream_new,
                &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                &generate_where_query_part_one_value_token_stream(&quote::quote!{"{}({} = ${})"}),
                &where_query_bind_one_value_token_stream,
            ),
            Filter::GreaterThan => (
                &pub_snake_case_token_stream,
                &comma_serde_deserialize_token_stream,
                &pub_value_t_token_stream,
                &proc_macro2_token_stream_new,
                &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                &generate_where_query_part_one_value_token_stream(&quote::quote!{"{}({} > ${})"}),
                &where_query_bind_one_value_token_stream,
            ),
            Filter::Between => (
                &proc_macro2_token_stream_new,
                &proc_macro2_token_stream_new,
                &quote::quote!{
                    start: T,
                    end: T,
                },
                &generate_enum_ident_try_new_error_named_token_stream(&quote::quote!{
                    StartMoreOrEqualToEnd {
                        start: T,
                        end: T,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                }),
                &quote::quote!{
                    start: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    end: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                },
                &quote::quote!{
                    match increment.checked_add(1) {
                        Some(first_value) => {
                            *increment = first_value;
                            match increment.checked_add(1) {
                                Some(second_value) => {
                                    *increment = second_value;
                                    let between_snake_case = naming::BetweenSnakeCase;
                                    let and_snake_case = naming::AndSnakeCase;
                                    Ok(format!("{}({column} {between_snake_case} ${first_value} {and_snake_case} ${second_value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                                }
                                None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                            }
                        }
                        None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                    }
                },
                &quote::quote!{
                    query = query.bind(self.start);//here change
                    query = query.bind(self.end);//here change
                    query
                }
            ),
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
                        logical_operator: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
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
            #enum_postgresql_type_where_element_filter_try_new_error_named_token_stream
            #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
            #impl_postgresql_type_self_where_filter_token_stream
        }
    };
    // let filter_array_token_stream = Filter::into_array().map(|element|generate_filters_token_stream(&element));
    let equal_token_stream = generate_filters_token_stream(&Filter::Equal);
    let greater_than_token_stream = generate_filters_token_stream(&Filter::GreaterThan);
    let generated = quote::quote! {
        // #(#filter_array_token_stream)*
        #equal_token_stream
        #greater_than_token_stream
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlTypeWhereElementFilters",
    //     &generated,
    // );
    generated.into()
}