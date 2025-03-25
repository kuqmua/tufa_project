#[proc_macro]
pub fn generate_where_element_filters(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let postgresql_type_token_stream = {
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
            //LengthEqual,
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
            //PositionEqual,
            //PositionGreaterThan,
            //PositionCaseSensitiveRegularExpression,
            //PositionCaseInsensitiveRegularExpression,
            //ContainsAllElementsOfArray,
            //ContainedInArray,
            //OverlapsWithArray,
            //AllElementsEqual,
            //ContainsElementGreaterThan,
            //AllElementsGreaterThan,
            //ContainsElementCaseSensitiveRegularExpression,
            //ContainsElementCaseInsensitiveRegularExpression,
            //AllElementsCaseSensitiveRegularExpression,
            //AllElementsCaseInsensitiveRegularExpression,
            //EqualSecondDimension,
        }
        #[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
        enum FilterInitializedWithTryNew {
            Between,
            In,
            CaseSensitiveRegularExpression,
            CaseInsensitiveRegularExpression,
            LengthMoreThan,
            RangeLength,
        }
        impl std::convert::TryFrom<&Filter> for FilterInitializedWithTryNew {
            type Error = ();
            fn try_from(value: &Filter) -> Result<Self, Self::Error> {
                match &value {
                    Filter::Equal => Err(()),
                    Filter::GreaterThan => Err(()),
                    Filter::Between => Ok(Self::Between),
                    Filter::In => Ok(Self::In),
                    Filter::CaseSensitiveRegularExpression => Ok(Self::CaseSensitiveRegularExpression),
                    Filter::CaseInsensitiveRegularExpression => Ok(Self::CaseInsensitiveRegularExpression),
                    Filter::Before => Err(()),
                    Filter::CurrentDate => Err(()),
                    Filter::GreaterThanCurrentDate => Err(()),
                    Filter::CurrentTimestamp => Err(()),
                    Filter::GreaterThanCurrentTimestamp => Err(()),
                    Filter::CurrentTime => Err(()),
                    Filter::GreaterThanCurrentTime => Err(()),
                    Filter::LengthMoreThan => Ok(Self::LengthMoreThan),
                    Filter::EqualToEncodedStringRepresentation => Err(()),
                    Filter::ValueIsContainedWithinRange => Err(()),
                    Filter::ContainsAnotherRange => Err(()),
                    Filter::StrictlyToLeftOfRange => Err(()),
                    Filter::StrictlyToRightOfRange => Err(()),
                    Filter::IncludedLowerBound => Err(()),
                    Filter::ExcludedUpperBound => Err(()),
                    Filter::GreaterThanLowerBound => Err(()),
                    Filter::OverlapWithRange => Err(()),
                    Filter::AdjacentWithRange => Err(()),
                    Filter::RangeLength => Ok(Self::RangeLength),
                }
            }
        }
        let generate_filters_token_stream = |filter: &Filter|{
            let query_snake_case = naming::QuerySnakeCase;
            let ident = naming::parameter::PostgresqlTypeWhereElementSelfUpperCamelCase::from_display(&filter);
            let ident_try_new_error_named = naming::parameter::PostgresqlTypeWhereElementSelfTryNewErrorNamedUpperCamelCase::from_display(&filter);
            let t_token_stream = quote::quote!{T};
            let t_annotation_generic_token_stream = quote::quote!{<#t_token_stream>};
            let std_vec_vec_t_token_stream = &quote::quote!{std::vec::Vec<T>};
            let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
            let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
            let core_default_default_default_token_stream = quote::quote!{
                ::core::default::Default::default()
            };
            let path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = quote::quote!{
                crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
            };
            let value_t_token_stream = quote::quote!{value: T};
            let pub_value_t_token_stream = quote::quote!{pub value: #t_token_stream};
            let value_std_vec_vec_t_token_stream = quote::quote!{value: #std_vec_vec_t_token_stream};
            let value_std_primitive_i32_token_stream = quote::quote!{value: #std_primitive_i32_token_stream};
            enum ShouldAddDeclarationOfStructIdentGeneric {
                True,
                False,
            }
            struct Field<'a> {
                field_name: &'a dyn std::fmt::Display,
                field_type: &'a dyn quote::ToTokens,
            }
            let value_code_default_token_stream = quote::quote!{
                value: #core_default_default_default_token_stream
            };
            let value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = quote::quote!{
                value: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
            };
            let generate_where_query_part_zero_value_token_stream = |format_handle_token_stream: &dyn quote::ToTokens|{
                quote::quote!{
                    Ok(format!(#format_handle_token_stream, &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column))
                }
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
            let generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream = |value: &std::primitive::str|{
                generate_quotes::double_quotes_token_stream(&format!("{{}}({{}} {value} ${{}})"))
            };
            let where_query_bind_one_value_token_stream = quote::quote!{
                query = query.bind(self.value);
                query
            };
            let (
                should_add_declaration_of_struct_ident_generic,
                struct_additional_fields_token_stream,
                impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream,
                where_query_part_content_token_stream,
                where_query_bind_content_token_stream,
            ) = match &filter {
                Filter::Equal => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("=")),
                    &where_query_bind_one_value_token_stream,
                ),
                Filter::GreaterThan => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(">")),
                    &where_query_bind_one_value_token_stream,
                ),
                Filter::Between => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &quote::quote!{
                        start: T,
                        end: T,
                    },
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
                Filter::In => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &value_std_vec_vec_t_token_stream,
                    &quote::quote!{
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream]
                    },
                    &quote::quote!{
                        let mut acc = std::string::String::default();
                        for element in &self.value {
                            match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    acc.push_str(&format!("${},", value));
                                }
                                None => {
                                    return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                                }
                            }
                        }
                        let _ = acc.pop();
                        let in_snake_case = naming::InSnakeCase;
                        Ok(format!("{}({} {in_snake_case} ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, acc))
                    },
                    &quote::quote!{
                        for element in self.value {
                            query = query.bind(element);
                        }
                        query
                    }
                ),
                Filter::CaseSensitiveRegularExpression => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("~")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::CaseInsensitiveRegularExpression => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("~*")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::Before => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("<")),
                    &where_query_bind_one_value_token_stream,
                ),
                Filter::CurrentDate => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} = current_date)"}),
                    &quote::quote!{#query_snake_case},
                ),
                Filter::GreaterThanCurrentDate => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} > current_date)"}),
                    &quote::quote!{#query_snake_case},
                ),
                Filter::CurrentTimestamp => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} = current_timestamp)"}),
                    &quote::quote!{#query_snake_case},
                ),
                Filter::GreaterThanCurrentTimestamp => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} > current_timestamp)"}),
                    &quote::quote!{#query_snake_case},
                ),
                Filter::CurrentTime => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} = current_time)"}),
                    &quote::quote!{#query_snake_case},
                ),
                Filter::GreaterThanCurrentTime => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} > current_time)"}),
                    &quote::quote!{#query_snake_case},
                ),
                //Filter::LengthEqual => todo!(),
                Filter::LengthMoreThan => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &value_std_primitive_i32_token_stream,
                    &value_code_default_token_stream,
                    &generate_where_query_part_one_value_token_stream(&quote::quote!{"{}(length({}) > ${})"}),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::EqualToEncodedStringRepresentation => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &quote::quote!{
                        pub encode_format: crate::postgresql_type::EncodeFormat,
                        pub encoded_string_representation: T,
                    },
                    &quote::quote!{
                        encode_format: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                        encoded_string_representation: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    },
                    &quote::quote!{
                        match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                Ok(format!("{}(encode({}, '{}') = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, &self.encode_format, increment))
                            }
                            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                        }
                    },
                    &quote::quote!{
                        query = query.bind(self.encoded_string_representation);
                        query
                    }
                ),
                Filter::ValueIsContainedWithinRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::ContainsAnotherRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::StrictlyToLeftOfRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&<")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::StrictlyToRightOfRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&>")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::IncludedLowerBound => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&quote::quote!{"{}(lower({}) = ${})"}),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::ExcludedUpperBound => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&quote::quote!{"{}(upper({}) = ${})"}),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::GreaterThanLowerBound => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(">")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::OverlapWithRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&&")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::AdjacentWithRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("-|-")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::RangeLength => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &value_std_primitive_i32_token_stream,
                    &value_code_default_token_stream,
                    &quote::quote!{
                        match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                Ok(format!("{}(upper({}) - lower({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, column, increment))
                            }
                            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                        }
                    },
                    &where_query_bind_one_value_token_stream
                ),
                //Filter:://BitVecPositionEqual => todo!(),
                //Filter::PositionEqual => todo!(),
                //Filter::PositionGreaterThan => todo!(),
                //Filter::PositionCaseSensitiveRegularExpression => todo!(),
                //Filter::PositionCaseInsensitiveRegularExpression => todo!(),
                //Filter::ContainsAllElementsOfArray => todo!(),
                //Filter::ContainedInArray => todo!(),
                //Filter::OverlapsWithArray => todo!(),
                //Filter::AllElementsEqual => todo!(),
                //Filter::ContainsElementGreaterThan => todo!(),
                //Filter::AllElementsGreaterThan => todo!(),
                //Filter::ContainsElementCaseSensitiveRegularExpression => todo!(),
                //Filter::ContainsElementCaseInsensitiveRegularExpression => todo!(),
                //Filter::AllElementsCaseSensitiveRegularExpression => todo!(),
                //Filter::AllElementsCaseInsensitiveRegularExpression => todo!(),
                //Filter::EqualSecondDimension => todo!(),
            };
            let filter_initialized_with_try_new_result = FilterInitializedWithTryNew::try_from(filter);
            let struct_token_stream = {
                let maybe_pub_token_stream: &dyn quote::ToTokens = if filter_initialized_with_try_new_result.is_ok() {
                    &proc_macro2_token_stream_new
                }
                else {
                    &naming::PubSnakeCase
                };
                let maybe_derive_serde_serialize_for_ident_struct_token_stream: &dyn quote::ToTokens = if filter_initialized_with_try_new_result.is_ok() {
                    &proc_macro2_token_stream_new
                }
                else {
                    &quote::quote!{, serde::Deserialize}
                };
                let maybe_declaration_of_struct_ident_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
                };
                quote::quote! {
                    #[derive(Debug, Clone, PartialEq, serde::Serialize #maybe_derive_serde_serialize_for_ident_struct_token_stream )]
                    pub struct #ident #maybe_declaration_of_struct_ident_generic_token_stream {
                        #maybe_pub_token_stream logical_operator: crate::LogicalOperator,
                        #struct_additional_fields_token_stream
                    }
                }
            };
            let maybe_try_new_logic_token_stream = match filter_initialized_with_try_new_result {
                Ok(value) => {
                    let value_std_primitive_i32_field = Field {
                        field_name: &naming::ValueSnakeCase,
                        field_type: &std_primitive_i32_token_stream,//todo i32 or i64 or something between? or more? or less?
                    };
                    let value_t_field = Field {
                        field_name: &naming::ValueSnakeCase,
                        field_type: &t_token_stream,
                    };
                    enum ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed {
                        True,
                        False,
                    }
                    let generate_maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream = |
                        should_add_declaration_of_generic_parameter_to_ident_try_new_error_named: &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed
                    |-> &dyn quote::ToTokens{
                        match &should_add_declaration_of_generic_parameter_to_ident_try_new_error_named {
                            ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True => &t_annotation_generic_token_stream,
                            ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False => &proc_macro2_token_stream_new
                        }
                    };
                    let (
                        should_add_declaration_of_generic_parameter_to_ident_try_new_error_named,
                        enum_ident_try_new_error_named_content_token_stream,
                        should_add_declaration_of_struct_ident_generic,
                        generic_requirements_token_stream,
                        additional_input_parameters_token_stream,
                        impl_try_new_for_ident_content_token_stream,
                        option_additional_traits_annotations_token_stream,
                        additional_fields
                    ) = match &value {
                        FilterInitializedWithTryNew::Between => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True,
                            &quote::quote!{
                                StartMoreOrEqualToEnd {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    start: T,
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    end: T,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::True,
                            &quote::quote!{: std::cmp::PartialOrd},
                            &quote::quote!{
                                start: T,
                                end: T,
                            },
                            &quote::quote!{
                                if start < end {//removed .0
                                    Ok(Self {
                                        logical_operator,
                                        start,
                                        end
                                    })
                                } else {
                                    Err(#ident_try_new_error_named::StartMoreOrEqualToEnd {
                                        start,
                                        end,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            },
                            Some(quote::quote!{+ std::cmp::PartialOrd}),
                            &vec![
                                Field {
                                    field_name: &naming::StartSnakeCase,
                                    field_type: &t_token_stream,
                                },
                                Field {
                                    field_name: &naming::EndSnakeCase,
                                    field_type: &t_token_stream,
                                },
                            ]
                        ),
                        FilterInitializedWithTryNew::In => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True,
                            &quote::quote!{
                                IsEmpty {
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                NotUnique {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: T,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::True,
                            &quote::quote!{: PartialEq + Clone},
                            &value_std_vec_vec_t_token_stream,
                            &quote::quote!{
                                if value.is_empty() {
                                    return Err(#ident_try_new_error_named::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
                                }
                                {
                                    let mut acc = vec![];
                                    for element in &value {
                                        if !acc.contains(&element) {
                                            acc.push(element);
                                        } else {
                                            return Err(#ident_try_new_error_named::NotUnique {
                                                value: element.clone(),
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    }
                                }
                                Ok(Self { logical_operator, value })
                            },
                            Some(quote::quote!{+ std::cmp::PartialOrd + Clone}),
                            &vec![
                                Field {
                                    field_name: &naming::ValueSnakeCase,
                                    field_type: &std_vec_vec_t_token_stream,
                                },
                            ]
                        ),
                        FilterInitializedWithTryNew::CaseSensitiveRegularExpression => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote!{
                                //todo
                                IsEmpty {
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::True,
                            &quote::quote!{: IsEmpty},
                            &value_t_token_stream,
                            &quote::quote!{
                                if !IsEmpty::is_empty(&value) {
                                    Ok(Self { logical_operator, value })
                                } else {
                                    Err(#ident_try_new_error_named::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
                                }
                            },
                            Some(quote::quote!{+ IsEmpty}),
                            &vec![value_t_field]
                        ),
                        FilterInitializedWithTryNew::CaseInsensitiveRegularExpression => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote!{
                                //todo
                                IsEmpty {
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::True,
                            &quote::quote!{: IsEmpty},
                            &value_t_token_stream,
                            &quote::quote!{
                                if !IsEmpty::is_empty(&value) {
                                    Ok(Self { logical_operator, value })
                                } else {
                                    Err(#ident_try_new_error_named::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
                                }
                            },
                            Some(quote::quote!{+ IsEmpty}),
                            &vec![value_t_field]
                        ),
                        FilterInitializedWithTryNew::LengthMoreThan => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote!{
                                LengthIsNegative {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: #std_primitive_i32_token_stream,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::False,
                            &proc_macro2_token_stream_new,
                            &value_std_primitive_i32_token_stream,
                            &quote::quote!{
                                if value >= 0 {
                                    Ok(Self { logical_operator, value })
                                } else {
                                    Err(#ident_try_new_error_named::LengthIsNegative {
                                        value,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            },
                            None,
                            &vec![value_std_primitive_i32_field]
                        ),
                        FilterInitializedWithTryNew::RangeLength => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote!{
                                LengthIsNegativeOrZero {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: #std_primitive_i32_token_stream,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::False,
                            &proc_macro2_token_stream_new,
                            &value_std_primitive_i32_token_stream,
                            &quote::quote!{
                                if value > 0 {
                                    Ok(Self { logical_operator, value })
                                } else {
                                    Err(#ident_try_new_error_named::LengthIsNegativeOrZero {
                                        value,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            },
                            None,
                            &vec![value_std_primitive_i32_field]
                        )
                    };
                    let enum_ident_try_new_error_named_token_stream = {
                        let maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream = generate_maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream(
                            &should_add_declaration_of_generic_parameter_to_ident_try_new_error_named
                        );
                        quote::quote!{
                            #[derive(
                                Debug,
                                Clone,
                                serde::Serialize,
                                serde::Deserialize,
                                thiserror::Error,
                                error_occurence_lib::ErrorOccurence,
                            )]
                            pub enum #ident_try_new_error_named #maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream {
                                #enum_ident_try_new_error_named_content_token_stream
                            }
                        }
                    };
                    let impl_try_new_for_ident_token_stream = {
                        let impl_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                            ShouldAddDeclarationOfStructIdentGeneric::True => &quote::quote!{<T #generic_requirements_token_stream>},
                            ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
                        };
                        let ident_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                            ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                            ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
                        };
                        let maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream = generate_maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream(
                            &should_add_declaration_of_generic_parameter_to_ident_try_new_error_named
                        );
                        quote::quote!{
                            impl #impl_generic_token_stream #ident #ident_generic_token_stream {
                                fn try_new(
                                    logical_operator: crate::LogicalOperator,
                                    #additional_input_parameters_token_stream
                                ) -> Result<Self, #ident_try_new_error_named #maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream> {
                                    #impl_try_new_for_ident_content_token_stream
                                }
                            }
                        }
                    };
                    let impl_serde_deserialize_for_ident_token_stream = {
                        let maybe_impl_generic_token_stream: &dyn quote::ToTokens = if option_additional_traits_annotations_token_stream.is_some() {
                            &quote::quote!{, T}
                        }
                        else {
                            &proc_macro2_token_stream_new
                        };
                        let maybe_ident_generic_token_stream: &dyn quote::ToTokens = if option_additional_traits_annotations_token_stream.is_some() {
                            &t_annotation_generic_token_stream
                        }
                        else {
                            &proc_macro2_token_stream_new
                        };
                        let maybe_where_generic_trait_annotation_token_stream: &dyn quote::ToTokens = match &option_additional_traits_annotations_token_stream {
                            Some(value) => &quote::quote!{
                                where
                                    T: std::fmt::Debug + _serde::Deserialize<'de> #value,
                            },
                            None => &proc_macro2_token_stream_new
                        };
                        let maybe_struct_visitor_where_generic_trait_annotation_token_stream = if option_additional_traits_annotations_token_stream.is_some() {
                            quote::quote!{
                                where
                                    T: _serde::Deserialize<'de>,
                            }
                        }
                        else {
                            proc_macro2::TokenStream::new()
                        };
                        let logical_operator_field = Field {
                            field_name: &naming::LogicalOperatorSnakeCase,
                            field_type: &quote::quote! {crate::LogicalOperator},
                        };
                        let fields = {
                            let mut acc = vec![&logical_operator_field];
                            for element in additional_fields {
                                acc.push(element);
                            }
                            acc
                        };
                        let (
                            struct_ident_double_quotes_token_stream,
                            struct_ident_with_number_of_elements_double_quotes_token_stream,
                            ident_double_quotes_token_stream
                        ) = postgresql_crud_macros_common::generate_serde_deserialize_double_quotes_token_stream(
                            &ident,
                            fields.len()
                        );
                        let enum_field_fields_token_stream = {
                            let value = fields.iter().enumerate().map(|(index, _)| format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap());
                            quote::quote! {#(#value),*}
                        };
                        let visit_u64_match_variants_token_stream = fields.iter().enumerate().map(|(index, _)| format!("{index}u64 => _serde::__private::Ok(__Field::__field{index})").parse::<proc_macro2::TokenStream>().unwrap());
                        let visit_str_match_variants_token_stream = fields
                            .iter()
                            .enumerate()
                            .map(|(index, element)| format!("{} => _serde::__private::Ok(__Field::__field{index})", generate_quotes::double_quotes_stringified(&element.field_name)).parse::<proc_macro2::TokenStream>().unwrap());
                        let visit_bytes_match_variants_token_stream = fields
                            .iter()
                            .enumerate()
                            .map(|(index, element)| format!("{} => _serde::__private::Ok(__Field::__field{index})", generate_quotes::binary_double_quotes_stringified(&element.field_name)).parse::<proc_macro2::TokenStream>().unwrap());
                        let visit_seq_initialization_token_stream = fields.iter().enumerate().map(|(index, element)| {
                            let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                            let element_field_type_token_stream = &element.field_type;
                            let index_usize_token_stream = format!("{index}usize").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                let #field_index_token_stream = match _serde::de::SeqAccess::next_element::<
                                    #element_field_type_token_stream,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                #index_usize_token_stream,
                                                &#struct_ident_with_number_of_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                            }
                        });
                        let visit_map_declaration_token_stream = fields.iter().enumerate().map(|(index, element)| {
                            let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                            let element_field_type_token_stream = &element.field_type;
                            quote::quote! {
                                let mut #field_index_token_stream: _serde::__private::Option<
                                    #element_field_type_token_stream,
                                > = _serde::__private::None;
                            }
                        });
                        let visit_map_match_variants_token_stream = fields.iter().enumerate().map(|(index, element)| {
                            let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                            let element_field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_name);
                            let element_field_type_token_stream = &element.field_type;
                            quote::quote! {
                                __Field::#field_index_token_stream => {
                                    if _serde::__private::Option::is_some(&#field_index_token_stream) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                #element_field_name_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                    #field_index_token_stream = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            #element_field_type_token_stream,
                                        >(&mut __map)?,
                                    );
                                }
                            }
                        });
                        let visit_map_initialization_token_stream = fields.iter().enumerate().map(|(index, element)| {
                            let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                            let element_field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_name);
                            quote::quote! {
                                let #field_index_token_stream = match #field_index_token_stream {
                                    _serde::__private::Some(#field_index_token_stream) => #field_index_token_stream,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field(#element_field_name_double_quotes_token_stream)?
                                    }
                                };
                            }
                        });
                        let field_names_double_quotes_token_stream = fields.iter().map(|element| generate_quotes::double_quotes_token_stream(&element.field_name));
                        let try_new_token_stream = quote::quote! {
                            match #ident::try_new(#enum_field_fields_token_stream) {
                                Ok(value) => _serde::__private::Ok(value),
                                Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                            }
                        };
                        quote::quote!{
                            const _: () = {
                                #[allow(unused_extern_crates, clippy::useless_attribute)]
                                extern crate serde as _serde;
                                #[automatically_derived]
                                impl<'de #maybe_impl_generic_token_stream> _serde::Deserialize<'de> for #ident #maybe_ident_generic_token_stream
                                #maybe_where_generic_trait_annotation_token_stream
                                {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        #[allow(non_camel_case_types)]
                                        #[doc(hidden)]
                                        enum __Field {
                                            #enum_field_fields_token_stream,
                                            __ignore,
                                        }
                                        #[doc(hidden)]
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::__private::Formatter<'_>,
                                            ) -> _serde::__private::fmt::Result {
                                                _serde::__private::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::__private::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    #(#visit_u64_match_variants_token_stream),*,
                                                    _ => _serde::__private::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::__private::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    #(#visit_str_match_variants_token_stream),*,
                                                    _ => _serde::__private::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::__private::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    #(#visit_bytes_match_variants_token_stream),*,
                                                    _ => _serde::__private::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        #[doc(hidden)]
                                        struct __Visitor<'de #maybe_impl_generic_token_stream>
                                        #maybe_struct_visitor_where_generic_trait_annotation_token_stream
                                        {
                                            marker: _serde::__private::PhantomData<
                                                #ident #maybe_ident_generic_token_stream,
                                            >,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de #maybe_impl_generic_token_stream> _serde::de::Visitor<'de> for __Visitor<'de #maybe_impl_generic_token_stream>
                                        #maybe_where_generic_trait_annotation_token_stream
                                        {
                                            type Value = #ident #maybe_ident_generic_token_stream;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::__private::Formatter<'_>,
                                            ) -> _serde::__private::fmt::Result {
                                                _serde::__private::Formatter::write_str(
                                                    __formatter,
                                                    #struct_ident_double_quotes_token_stream,
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                    
                                                #(#visit_seq_initialization_token_stream)*
                                                #try_new_token_stream
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                #(#visit_map_declaration_token_stream)*

                                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                                    __Field,
                                                >(&mut __map)? {
                                                    match __key {
                                                        #(#visit_map_match_variants_token_stream)*
                                                        _ => {
                                                            let _ = _serde::de::MapAccess::next_value::<
                                                                _serde::de::IgnoredAny,
                                                            >(&mut __map)?;
                                                        }
                                                    }
                                                }
                                                #(#visit_map_initialization_token_stream)*
                                                #try_new_token_stream
                                            }
                                        }
                                        #[doc(hidden)]
                                        const FIELDS: &'static [&'static str] = &[#(#field_names_double_quotes_token_stream),*];
                                        _serde::Deserializer::deserialize_struct(
                                            __deserializer,
                                            #ident_double_quotes_token_stream,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::__private::PhantomData::<
                                                    #ident #maybe_ident_generic_token_stream,
                                                >,
                                                lifetime: _serde::__private::PhantomData,
                                            },
                                        )
                                    }
                                }
                            };
                        }
                    };
                    quote::quote!{
                        #enum_ident_try_new_error_named_token_stream
                        #impl_try_new_for_ident_token_stream
                        #impl_serde_deserialize_for_ident_token_stream
                    }
                },
                Err(_) => proc_macro2::TokenStream::new()
            };
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = postgresql_crud_macros_common::generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                &match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => quote::quote!{<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>},
                    ShouldAddDeclarationOfStructIdentGeneric::False => proc_macro2::TokenStream::new()
                },
                &postgresql_crud_macros_common::PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::Crate,
                &ident,
                match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new
                },
                &{
                    quote::quote!{
                        Self {
                            logical_operator: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                            #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream
                        }
                    }
                },
            );
            let impl_postgresql_type_self_where_filter_token_stream = postgresql_crud_macros_common::impl_postgresql_type_self_where_filter_for_ident_token_stream(
                &{
                    let maybe_t_additional_traits_for_postgresql_type_self_where_filter_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                        ShouldAddDeclarationOfStructIdentGeneric::True => &quote::quote!{, T: sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + 'a + std::marker::Send},
                        ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new
                    };
                    quote::quote!{<'a #maybe_t_additional_traits_for_postgresql_type_self_where_filter_token_stream>}
                },
                &ident,
                &match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new
                },
                &where_query_part_content_token_stream,
                &where_query_bind_content_token_stream,
                &postgresql_crud_macros_common::PostgresqlTypeSelfWhereFilterPath::Crate,
            );
            quote::quote! {
                #struct_token_stream
                #maybe_try_new_logic_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                #impl_postgresql_type_self_where_filter_token_stream
            }
        };
        // let filter_array_token_stream = Filter::into_array().map(|element|generate_filters_token_stream(&element));
        let equal_token_stream = generate_filters_token_stream(&Filter::Equal);
        let greater_than_token_stream = generate_filters_token_stream(&Filter::GreaterThan);
        let between_token_stream = generate_filters_token_stream(&Filter::Between);
        let in_token_stream = generate_filters_token_stream(&Filter::In);
        let case_sensitive_regular_expression_token_stream = generate_filters_token_stream(&Filter::CaseSensitiveRegularExpression);
        let case_insensitive_regular_expression_token_stream = generate_filters_token_stream(&Filter::CaseInsensitiveRegularExpression);
        let before_token_stream = generate_filters_token_stream(&Filter::Before);
        let current_date_token_stream = generate_filters_token_stream(&Filter::CurrentDate);
        let greater_than_current_date_token_stream = generate_filters_token_stream(&Filter::GreaterThanCurrentDate);
        let current_timestamp_token_stream = generate_filters_token_stream(&Filter::CurrentTimestamp);
        let greater_than_current_timestamp_token_stream = generate_filters_token_stream(&Filter::GreaterThanCurrentTimestamp);
        let current_time_token_stream = generate_filters_token_stream(&Filter::CurrentTime);
        let greater_than_current_time_token_stream = generate_filters_token_stream(&Filter::GreaterThanCurrentTime);
        let length_more_than_token_stream = generate_filters_token_stream(&Filter::LengthMoreThan);
        let equal_to_encoded_string_representation_token_stream = generate_filters_token_stream(&Filter::EqualToEncodedStringRepresentation);
        let value_is_contained_within_range_token_stream = generate_filters_token_stream(&Filter::ValueIsContainedWithinRange);
        let contains_another_range_token_stream = generate_filters_token_stream(&Filter::ContainsAnotherRange);
        let strictly_to_left_of_range_token_stream = generate_filters_token_stream(&Filter::StrictlyToLeftOfRange);
        let strictly_to_right_of_range_token_stream = generate_filters_token_stream(&Filter::StrictlyToRightOfRange);
        let included_lower_bound_token_stream = generate_filters_token_stream(&Filter::IncludedLowerBound);
        let excluded_upper_bound_token_stream = generate_filters_token_stream(&Filter::ExcludedUpperBound);
        let greater_than_lower_bound_token_stream = generate_filters_token_stream(&Filter::GreaterThanLowerBound);
        let overlap_with_range_token_stream = generate_filters_token_stream(&Filter::OverlapWithRange);
        let adjacent_with_range_token_stream = generate_filters_token_stream(&Filter::AdjacentWithRange);
        let range_length_token_stream = generate_filters_token_stream(&Filter::RangeLength);
        // let _token_stream = generate_filters_token_stream(&Filter::);
        quote::quote! {
            //#(#filter_array_token_stream)*

            #equal_token_stream
            #greater_than_token_stream
            #between_token_stream
            #in_token_stream
            #case_sensitive_regular_expression_token_stream
            #case_insensitive_regular_expression_token_stream
            #before_token_stream
            #current_date_token_stream
            #greater_than_current_date_token_stream
            #current_timestamp_token_stream
            #greater_than_current_timestamp_token_stream
            #current_time_token_stream
            #greater_than_current_time_token_stream
            #length_more_than_token_stream
            #equal_to_encoded_string_representation_token_stream
            #value_is_contained_within_range_token_stream
            #contains_another_range_token_stream
            #strictly_to_left_of_range_token_stream
            #strictly_to_right_of_range_token_stream
            #included_lower_bound_token_stream
            #excluded_upper_bound_token_stream
            #greater_than_lower_bound_token_stream
            #overlap_with_range_token_stream
            #adjacent_with_range_token_stream
            #range_length_token_stream
            //#_token_stream
        }
    };
    let postgresql_json_type_token_stream = {
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
            //LengthEqual,
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
            //PositionEqual,
            //PositionGreaterThan,
            //PositionCaseSensitiveRegularExpression,
            //PositionCaseInsensitiveRegularExpression,
            //ContainsAllElementsOfArray,
            //ContainedInArray,
            //OverlapsWithArray,
            //AllElementsEqual,
            //ContainsElementGreaterThan,
            //AllElementsGreaterThan,
            //ContainsElementCaseSensitiveRegularExpression,
            //ContainsElementCaseInsensitiveRegularExpression,
            //AllElementsCaseSensitiveRegularExpression,
            //AllElementsCaseInsensitiveRegularExpression,
            //EqualSecondDimension,
        }
        #[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
        enum FilterInitializedWithTryNew {
            Between,
            In,
            CaseSensitiveRegularExpression,
            CaseInsensitiveRegularExpression,
            LengthMoreThan,
            RangeLength,
        }
        impl std::convert::TryFrom<&Filter> for FilterInitializedWithTryNew {
            type Error = ();
            fn try_from(value: &Filter) -> Result<Self, Self::Error> {
                match &value {
                    Filter::Equal => Err(()),
                    Filter::GreaterThan => Err(()),
                    Filter::Between => Ok(Self::Between),
                    Filter::In => Ok(Self::In),
                    Filter::CaseSensitiveRegularExpression => Ok(Self::CaseSensitiveRegularExpression),
                    Filter::CaseInsensitiveRegularExpression => Ok(Self::CaseInsensitiveRegularExpression),
                    Filter::Before => Err(()),
                    Filter::CurrentDate => Err(()),
                    Filter::GreaterThanCurrentDate => Err(()),
                    Filter::CurrentTimestamp => Err(()),
                    Filter::GreaterThanCurrentTimestamp => Err(()),
                    Filter::CurrentTime => Err(()),
                    Filter::GreaterThanCurrentTime => Err(()),
                    Filter::LengthMoreThan => Ok(Self::LengthMoreThan),
                    Filter::EqualToEncodedStringRepresentation => Err(()),
                    Filter::ValueIsContainedWithinRange => Err(()),
                    Filter::ContainsAnotherRange => Err(()),
                    Filter::StrictlyToLeftOfRange => Err(()),
                    Filter::StrictlyToRightOfRange => Err(()),
                    Filter::IncludedLowerBound => Err(()),
                    Filter::ExcludedUpperBound => Err(()),
                    Filter::GreaterThanLowerBound => Err(()),
                    Filter::OverlapWithRange => Err(()),
                    Filter::AdjacentWithRange => Err(()),
                    Filter::RangeLength => Ok(Self::RangeLength),
                }
            }
        }
        let generate_filters_token_stream = |filter: &Filter|{
            let query_snake_case = naming::QuerySnakeCase;
            let ident = naming::parameter::PostgresqlTypeWhereElementSelfUpperCamelCase::from_display(&filter);
            let ident_try_new_error_named = naming::parameter::PostgresqlTypeWhereElementSelfTryNewErrorNamedUpperCamelCase::from_display(&filter);
            let t_token_stream = quote::quote!{T};
            let t_annotation_generic_token_stream = quote::quote!{<#t_token_stream>};
            let std_vec_vec_t_token_stream = &quote::quote!{std::vec::Vec<T>};
            let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
            let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
            let core_default_default_default_token_stream = quote::quote!{
                ::core::default::Default::default()
            };
            let path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = quote::quote!{
                crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
            };
            let value_t_token_stream = quote::quote!{value: T};
            let pub_value_t_token_stream = quote::quote!{pub value: #t_token_stream};
            let value_std_vec_vec_t_token_stream = quote::quote!{value: #std_vec_vec_t_token_stream};
            let value_std_primitive_i32_token_stream = quote::quote!{value: #std_primitive_i32_token_stream};
            enum ShouldAddDeclarationOfStructIdentGeneric {
                True,
                False,
            }
            struct Field<'a> {
                field_name: &'a dyn std::fmt::Display,
                field_type: &'a dyn quote::ToTokens,
            }
            let value_code_default_token_stream = quote::quote!{
                value: #core_default_default_default_token_stream
            };
            let value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = quote::quote!{
                value: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
            };
            let generate_where_query_part_zero_value_token_stream = |format_handle_token_stream: &dyn quote::ToTokens|{
                quote::quote!{
                    Ok(format!(#format_handle_token_stream, &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column))
                }
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
            let generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream = |value: &std::primitive::str|{
                generate_quotes::double_quotes_token_stream(&format!("{{}}({{}} {value} ${{}})"))
            };
            let where_query_bind_one_value_token_stream = quote::quote!{
                query = query.bind(self.value);
                query
            };
            let (
                should_add_declaration_of_struct_ident_generic,
                struct_additional_fields_token_stream,
                impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream,
                where_query_part_content_token_stream,
                where_query_bind_content_token_stream,
            ) = match &filter {
                Filter::Equal => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("=")),
                    &where_query_bind_one_value_token_stream,
                ),
                Filter::GreaterThan => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(">")),
                    &where_query_bind_one_value_token_stream,
                ),
                Filter::Between => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &quote::quote!{
                        start: T,
                        end: T,
                    },
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
                Filter::In => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &value_std_vec_vec_t_token_stream,
                    &quote::quote!{
                        value: vec![#path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream]
                    },
                    &quote::quote!{
                        let mut acc = std::string::String::default();
                        for element in &self.value {
                            match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    acc.push_str(&format!("${},", value));
                                }
                                None => {
                                    return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                                }
                            }
                        }
                        let _ = acc.pop();
                        let in_snake_case = naming::InSnakeCase;
                        Ok(format!("{}({} {in_snake_case} ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, acc))
                    },
                    &quote::quote!{
                        for element in self.value {
                            query = query.bind(element);
                        }
                        query
                    }
                ),
                Filter::CaseSensitiveRegularExpression => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("~")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::CaseInsensitiveRegularExpression => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("~*")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::Before => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("<")),
                    &where_query_bind_one_value_token_stream,
                ),
                Filter::CurrentDate => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} = current_date)"}),
                    &quote::quote!{#query_snake_case},
                ),
                Filter::GreaterThanCurrentDate => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} > current_date)"}),
                    &quote::quote!{#query_snake_case},
                ),
                Filter::CurrentTimestamp => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} = current_timestamp)"}),
                    &quote::quote!{#query_snake_case},
                ),
                Filter::GreaterThanCurrentTimestamp => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} > current_timestamp)"}),
                    &quote::quote!{#query_snake_case},
                ),
                Filter::CurrentTime => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} = current_time)"}),
                    &quote::quote!{#query_snake_case},
                ),
                Filter::GreaterThanCurrentTime => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &proc_macro2_token_stream_new,
                    &proc_macro2_token_stream_new,
                    &generate_where_query_part_zero_value_token_stream(&quote::quote!{"{}({} > current_time)"}),
                    &quote::quote!{#query_snake_case},
                ),
                //Filter::LengthEqual => todo!(),
                Filter::LengthMoreThan => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &value_std_primitive_i32_token_stream,
                    &value_code_default_token_stream,
                    &generate_where_query_part_one_value_token_stream(&quote::quote!{"{}(length({}) > ${})"}),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::EqualToEncodedStringRepresentation => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &quote::quote!{
                        pub encode_format: crate::postgresql_type::EncodeFormat,
                        pub encoded_string_representation: T,
                    },
                    &quote::quote!{
                        encode_format: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                        encoded_string_representation: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    },
                    &quote::quote!{
                        match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                Ok(format!("{}(encode({}, '{}') = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, &self.encode_format, increment))
                            }
                            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                        }
                    },
                    &quote::quote!{
                        query = query.bind(self.encoded_string_representation);
                        query
                    }
                ),
                Filter::ValueIsContainedWithinRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::ContainsAnotherRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("@>")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::StrictlyToLeftOfRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&<")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::StrictlyToRightOfRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&>")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::IncludedLowerBound => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&quote::quote!{"{}(lower({}) = ${})"}),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::ExcludedUpperBound => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&quote::quote!{"{}(upper({}) = ${})"}),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::GreaterThanLowerBound => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream(">")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::OverlapWithRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("&&")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::AdjacentWithRange => (
                    ShouldAddDeclarationOfStructIdentGeneric::True,
                    &pub_value_t_token_stream,
                    &value_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &generate_where_query_part_one_value_token_stream(&generate_format_handle_8bbcc2f2_f3a1_4aed_9c46_2992ea2e9e9b_token_stream("-|-")),
                    &where_query_bind_one_value_token_stream
                ),
                Filter::RangeLength => (
                    ShouldAddDeclarationOfStructIdentGeneric::False,
                    &value_std_primitive_i32_token_stream,
                    &value_code_default_token_stream,
                    &quote::quote!{
                        match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                Ok(format!("{}(upper({}) - lower({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, column, increment))
                            }
                            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                        }
                    },
                    &where_query_bind_one_value_token_stream
                ),
                //Filter:://BitVecPositionEqual => todo!(),
                //Filter::PositionEqual => todo!(),
                //Filter::PositionGreaterThan => todo!(),
                //Filter::PositionCaseSensitiveRegularExpression => todo!(),
                //Filter::PositionCaseInsensitiveRegularExpression => todo!(),
                //Filter::ContainsAllElementsOfArray => todo!(),
                //Filter::ContainedInArray => todo!(),
                //Filter::OverlapsWithArray => todo!(),
                //Filter::AllElementsEqual => todo!(),
                //Filter::ContainsElementGreaterThan => todo!(),
                //Filter::AllElementsGreaterThan => todo!(),
                //Filter::ContainsElementCaseSensitiveRegularExpression => todo!(),
                //Filter::ContainsElementCaseInsensitiveRegularExpression => todo!(),
                //Filter::AllElementsCaseSensitiveRegularExpression => todo!(),
                //Filter::AllElementsCaseInsensitiveRegularExpression => todo!(),
                //Filter::EqualSecondDimension => todo!(),
            };
            let filter_initialized_with_try_new_result = FilterInitializedWithTryNew::try_from(filter);
            let struct_token_stream = {
                let maybe_pub_token_stream: &dyn quote::ToTokens = if filter_initialized_with_try_new_result.is_ok() {
                    &proc_macro2_token_stream_new
                }
                else {
                    &naming::PubSnakeCase
                };
                let maybe_derive_serde_serialize_for_ident_struct_token_stream: &dyn quote::ToTokens = if filter_initialized_with_try_new_result.is_ok() {
                    &proc_macro2_token_stream_new
                }
                else {
                    &quote::quote!{, serde::Deserialize}
                };
                let maybe_declaration_of_struct_ident_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
                };
                quote::quote! {
                    #[derive(Debug, Clone, PartialEq, serde::Serialize #maybe_derive_serde_serialize_for_ident_struct_token_stream )]
                    pub struct #ident #maybe_declaration_of_struct_ident_generic_token_stream {
                        #maybe_pub_token_stream logical_operator: crate::LogicalOperator,
                        #struct_additional_fields_token_stream
                    }
                }
            };
            let maybe_try_new_logic_token_stream = match filter_initialized_with_try_new_result {
                Ok(value) => {
                    let value_std_primitive_i32_field = Field {
                        field_name: &naming::ValueSnakeCase,
                        field_type: &std_primitive_i32_token_stream,//todo i32 or i64 or something between? or more? or less?
                    };
                    let value_t_field = Field {
                        field_name: &naming::ValueSnakeCase,
                        field_type: &t_token_stream,
                    };
                    enum ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed {
                        True,
                        False,
                    }
                    let generate_maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream = |
                        should_add_declaration_of_generic_parameter_to_ident_try_new_error_named: &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed
                    |-> &dyn quote::ToTokens{
                        match &should_add_declaration_of_generic_parameter_to_ident_try_new_error_named {
                            ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True => &t_annotation_generic_token_stream,
                            ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False => &proc_macro2_token_stream_new
                        }
                    };
                    let (
                        should_add_declaration_of_generic_parameter_to_ident_try_new_error_named,
                        enum_ident_try_new_error_named_content_token_stream,
                        should_add_declaration_of_struct_ident_generic,
                        generic_requirements_token_stream,
                        additional_input_parameters_token_stream,
                        impl_try_new_for_ident_content_token_stream,
                        option_additional_traits_annotations_token_stream,
                        additional_fields
                    ) = match &value {
                        FilterInitializedWithTryNew::Between => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True,
                            &quote::quote!{
                                StartMoreOrEqualToEnd {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    start: T,
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    end: T,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::True,
                            &quote::quote!{: std::cmp::PartialOrd},
                            &quote::quote!{
                                start: T,
                                end: T,
                            },
                            &quote::quote!{
                                if start < end {//removed .0
                                    Ok(Self {
                                        logical_operator,
                                        start,
                                        end
                                    })
                                } else {
                                    Err(#ident_try_new_error_named::StartMoreOrEqualToEnd {
                                        start,
                                        end,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            },
                            Some(quote::quote!{+ std::cmp::PartialOrd}),
                            &vec![
                                Field {
                                    field_name: &naming::StartSnakeCase,
                                    field_type: &t_token_stream,
                                },
                                Field {
                                    field_name: &naming::EndSnakeCase,
                                    field_type: &t_token_stream,
                                },
                            ]
                        ),
                        FilterInitializedWithTryNew::In => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::True,
                            &quote::quote!{
                                IsEmpty {
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                NotUnique {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: T,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::True,
                            &quote::quote!{: PartialEq + Clone},
                            &value_std_vec_vec_t_token_stream,
                            &quote::quote!{
                                if value.is_empty() {
                                    return Err(#ident_try_new_error_named::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
                                }
                                {
                                    let mut acc = vec![];
                                    for element in &value {
                                        if !acc.contains(&element) {
                                            acc.push(element);
                                        } else {
                                            return Err(#ident_try_new_error_named::NotUnique {
                                                value: element.clone(),
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    }
                                }
                                Ok(Self { logical_operator, value })
                            },
                            Some(quote::quote!{+ std::cmp::PartialOrd + Clone}),
                            &vec![
                                Field {
                                    field_name: &naming::ValueSnakeCase,
                                    field_type: &std_vec_vec_t_token_stream,
                                },
                            ]
                        ),
                        FilterInitializedWithTryNew::CaseSensitiveRegularExpression => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote!{
                                //todo
                                IsEmpty {
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::True,
                            &quote::quote!{: IsEmpty},
                            &value_t_token_stream,
                            &quote::quote!{
                                if !IsEmpty::is_empty(&value) {
                                    Ok(Self { logical_operator, value })
                                } else {
                                    Err(#ident_try_new_error_named::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
                                }
                            },
                            Some(quote::quote!{+ IsEmpty}),
                            &vec![value_t_field]
                        ),
                        FilterInitializedWithTryNew::CaseInsensitiveRegularExpression => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote!{
                                //todo
                                IsEmpty {
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::True,
                            &quote::quote!{: IsEmpty},
                            &value_t_token_stream,
                            &quote::quote!{
                                if !IsEmpty::is_empty(&value) {
                                    Ok(Self { logical_operator, value })
                                } else {
                                    Err(#ident_try_new_error_named::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
                                }
                            },
                            Some(quote::quote!{+ IsEmpty}),
                            &vec![value_t_field]
                        ),
                        FilterInitializedWithTryNew::LengthMoreThan => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote!{
                                LengthIsNegative {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: #std_primitive_i32_token_stream,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::False,
                            &proc_macro2_token_stream_new,
                            &value_std_primitive_i32_token_stream,
                            &quote::quote!{
                                if value >= 0 {
                                    Ok(Self { logical_operator, value })
                                } else {
                                    Err(#ident_try_new_error_named::LengthIsNegative {
                                        value,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            },
                            None,
                            &vec![value_std_primitive_i32_field]
                        ),
                        FilterInitializedWithTryNew::RangeLength => (
                            &ShouldAddDeclarationOfGenericParameterToIdentTryNewErrorNamed::False,
                            &quote::quote!{
                                LengthIsNegativeOrZero {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    value: #std_primitive_i32_token_stream,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            &ShouldAddDeclarationOfStructIdentGeneric::False,
                            &proc_macro2_token_stream_new,
                            &value_std_primitive_i32_token_stream,
                            &quote::quote!{
                                if value > 0 {
                                    Ok(Self { logical_operator, value })
                                } else {
                                    Err(#ident_try_new_error_named::LengthIsNegativeOrZero {
                                        value,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            },
                            None,
                            &vec![value_std_primitive_i32_field]
                        )
                    };
                    let enum_ident_try_new_error_named_token_stream = {
                        let maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream = generate_maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream(
                            &should_add_declaration_of_generic_parameter_to_ident_try_new_error_named
                        );
                        quote::quote!{
                            #[derive(
                                Debug,
                                Clone,
                                serde::Serialize,
                                serde::Deserialize,
                                thiserror::Error,
                                error_occurence_lib::ErrorOccurence,
                            )]
                            pub enum #ident_try_new_error_named #maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream {
                                #enum_ident_try_new_error_named_content_token_stream
                            }
                        }
                    };
                    let impl_try_new_for_ident_token_stream = {
                        let impl_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                            ShouldAddDeclarationOfStructIdentGeneric::True => &quote::quote!{<T #generic_requirements_token_stream>},
                            ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
                        };
                        let ident_generic_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                            ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                            ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new,
                        };
                        let maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream = generate_maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream(
                            &should_add_declaration_of_generic_parameter_to_ident_try_new_error_named
                        );
                        quote::quote!{
                            impl #impl_generic_token_stream #ident #ident_generic_token_stream {
                                fn try_new(
                                    logical_operator: crate::LogicalOperator,
                                    #additional_input_parameters_token_stream
                                ) -> Result<Self, #ident_try_new_error_named #maybe_declaration_of_generic_parameter_to_ident_try_new_error_named_token_stream> {
                                    #impl_try_new_for_ident_content_token_stream
                                }
                            }
                        }
                    };
                    let impl_serde_deserialize_for_ident_token_stream = {
                        let maybe_impl_generic_token_stream: &dyn quote::ToTokens = if option_additional_traits_annotations_token_stream.is_some() {
                            &quote::quote!{, T}
                        }
                        else {
                            &proc_macro2_token_stream_new
                        };
                        let maybe_ident_generic_token_stream: &dyn quote::ToTokens = if option_additional_traits_annotations_token_stream.is_some() {
                            &t_annotation_generic_token_stream
                        }
                        else {
                            &proc_macro2_token_stream_new
                        };
                        let maybe_where_generic_trait_annotation_token_stream: &dyn quote::ToTokens = match &option_additional_traits_annotations_token_stream {
                            Some(value) => &quote::quote!{
                                where
                                    T: std::fmt::Debug + _serde::Deserialize<'de> #value,
                            },
                            None => &proc_macro2_token_stream_new
                        };
                        let maybe_struct_visitor_where_generic_trait_annotation_token_stream = if option_additional_traits_annotations_token_stream.is_some() {
                            quote::quote!{
                                where
                                    T: _serde::Deserialize<'de>,
                            }
                        }
                        else {
                            proc_macro2::TokenStream::new()
                        };
                        let logical_operator_field = Field {
                            field_name: &naming::LogicalOperatorSnakeCase,
                            field_type: &quote::quote! {crate::LogicalOperator},
                        };
                        let fields = {
                            let mut acc = vec![&logical_operator_field];
                            for element in additional_fields {
                                acc.push(element);
                            }
                            acc
                        };
                        let (
                            struct_ident_double_quotes_token_stream,
                            struct_ident_with_number_of_elements_double_quotes_token_stream,
                            ident_double_quotes_token_stream
                        ) = postgresql_crud_macros_common::generate_serde_deserialize_double_quotes_token_stream(
                            &ident,
                            fields.len()
                        );
                        let enum_field_fields_token_stream = {
                            let value = fields.iter().enumerate().map(|(index, _)| format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap());
                            quote::quote! {#(#value),*}
                        };
                        let visit_u64_match_variants_token_stream = fields.iter().enumerate().map(|(index, _)| format!("{index}u64 => _serde::__private::Ok(__Field::__field{index})").parse::<proc_macro2::TokenStream>().unwrap());
                        let visit_str_match_variants_token_stream = fields
                            .iter()
                            .enumerate()
                            .map(|(index, element)| format!("{} => _serde::__private::Ok(__Field::__field{index})", generate_quotes::double_quotes_stringified(&element.field_name)).parse::<proc_macro2::TokenStream>().unwrap());
                        let visit_bytes_match_variants_token_stream = fields
                            .iter()
                            .enumerate()
                            .map(|(index, element)| format!("{} => _serde::__private::Ok(__Field::__field{index})", generate_quotes::binary_double_quotes_stringified(&element.field_name)).parse::<proc_macro2::TokenStream>().unwrap());
                        let visit_seq_initialization_token_stream = fields.iter().enumerate().map(|(index, element)| {
                            let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                            let element_field_type_token_stream = &element.field_type;
                            let index_usize_token_stream = format!("{index}usize").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                let #field_index_token_stream = match _serde::de::SeqAccess::next_element::<
                                    #element_field_type_token_stream,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                #index_usize_token_stream,
                                                &#struct_ident_with_number_of_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                            }
                        });
                        let visit_map_declaration_token_stream = fields.iter().enumerate().map(|(index, element)| {
                            let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                            let element_field_type_token_stream = &element.field_type;
                            quote::quote! {
                                let mut #field_index_token_stream: _serde::__private::Option<
                                    #element_field_type_token_stream,
                                > = _serde::__private::None;
                            }
                        });
                        let visit_map_match_variants_token_stream = fields.iter().enumerate().map(|(index, element)| {
                            let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                            let element_field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_name);
                            let element_field_type_token_stream = &element.field_type;
                            quote::quote! {
                                __Field::#field_index_token_stream => {
                                    if _serde::__private::Option::is_some(&#field_index_token_stream) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                #element_field_name_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                    #field_index_token_stream = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            #element_field_type_token_stream,
                                        >(&mut __map)?,
                                    );
                                }
                            }
                        });
                        let visit_map_initialization_token_stream = fields.iter().enumerate().map(|(index, element)| {
                            let field_index_token_stream = format!("__field{index}").parse::<proc_macro2::TokenStream>().unwrap();
                            let element_field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_name);
                            quote::quote! {
                                let #field_index_token_stream = match #field_index_token_stream {
                                    _serde::__private::Some(#field_index_token_stream) => #field_index_token_stream,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field(#element_field_name_double_quotes_token_stream)?
                                    }
                                };
                            }
                        });
                        let field_names_double_quotes_token_stream = fields.iter().map(|element| generate_quotes::double_quotes_token_stream(&element.field_name));
                        let try_new_token_stream = quote::quote! {
                            match #ident::try_new(#enum_field_fields_token_stream) {
                                Ok(value) => _serde::__private::Ok(value),
                                Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                            }
                        };
                        quote::quote!{
                            const _: () = {
                                #[allow(unused_extern_crates, clippy::useless_attribute)]
                                extern crate serde as _serde;
                                #[automatically_derived]
                                impl<'de #maybe_impl_generic_token_stream> _serde::Deserialize<'de> for #ident #maybe_ident_generic_token_stream
                                #maybe_where_generic_trait_annotation_token_stream
                                {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        #[allow(non_camel_case_types)]
                                        #[doc(hidden)]
                                        enum __Field {
                                            #enum_field_fields_token_stream,
                                            __ignore,
                                        }
                                        #[doc(hidden)]
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::__private::Formatter<'_>,
                                            ) -> _serde::__private::fmt::Result {
                                                _serde::__private::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::__private::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    #(#visit_u64_match_variants_token_stream),*,
                                                    _ => _serde::__private::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::__private::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    #(#visit_str_match_variants_token_stream),*,
                                                    _ => _serde::__private::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::__private::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    #(#visit_bytes_match_variants_token_stream),*,
                                                    _ => _serde::__private::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        #[doc(hidden)]
                                        struct __Visitor<'de #maybe_impl_generic_token_stream>
                                        #maybe_struct_visitor_where_generic_trait_annotation_token_stream
                                        {
                                            marker: _serde::__private::PhantomData<
                                                #ident #maybe_ident_generic_token_stream,
                                            >,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de #maybe_impl_generic_token_stream> _serde::de::Visitor<'de> for __Visitor<'de #maybe_impl_generic_token_stream>
                                        #maybe_where_generic_trait_annotation_token_stream
                                        {
                                            type Value = #ident #maybe_ident_generic_token_stream;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::__private::Formatter<'_>,
                                            ) -> _serde::__private::fmt::Result {
                                                _serde::__private::Formatter::write_str(
                                                    __formatter,
                                                    #struct_ident_double_quotes_token_stream,
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                    
                                                #(#visit_seq_initialization_token_stream)*
                                                #try_new_token_stream
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                #(#visit_map_declaration_token_stream)*

                                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                                    __Field,
                                                >(&mut __map)? {
                                                    match __key {
                                                        #(#visit_map_match_variants_token_stream)*
                                                        _ => {
                                                            let _ = _serde::de::MapAccess::next_value::<
                                                                _serde::de::IgnoredAny,
                                                            >(&mut __map)?;
                                                        }
                                                    }
                                                }
                                                #(#visit_map_initialization_token_stream)*
                                                #try_new_token_stream
                                            }
                                        }
                                        #[doc(hidden)]
                                        const FIELDS: &'static [&'static str] = &[#(#field_names_double_quotes_token_stream),*];
                                        _serde::Deserializer::deserialize_struct(
                                            __deserializer,
                                            #ident_double_quotes_token_stream,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::__private::PhantomData::<
                                                    #ident #maybe_ident_generic_token_stream,
                                                >,
                                                lifetime: _serde::__private::PhantomData,
                                            },
                                        )
                                    }
                                }
                            };
                        }
                    };
                    quote::quote!{
                        #enum_ident_try_new_error_named_token_stream
                        #impl_try_new_for_ident_token_stream
                        #impl_serde_deserialize_for_ident_token_stream
                    }
                },
                Err(_) => proc_macro2::TokenStream::new()
            };
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = postgresql_crud_macros_common::generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                &match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => quote::quote!{<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>},
                    ShouldAddDeclarationOfStructIdentGeneric::False => proc_macro2::TokenStream::new()
                },
                &postgresql_crud_macros_common::PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::Crate,
                &ident,
                match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new
                },
                &{
                    quote::quote!{
                        Self {
                            logical_operator: #path_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                            #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_additional_fields_token_stream
                        }
                    }
                },
            );
            let impl_postgresql_type_self_where_filter_token_stream = postgresql_crud_macros_common::impl_postgresql_type_self_where_filter_for_ident_token_stream(
                &{
                    let maybe_t_additional_traits_for_postgresql_type_self_where_filter_token_stream: &dyn quote::ToTokens = match &should_add_declaration_of_struct_ident_generic {
                        ShouldAddDeclarationOfStructIdentGeneric::True => &quote::quote!{, T: sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + 'a + std::marker::Send},
                        ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new
                    };
                    quote::quote!{<'a #maybe_t_additional_traits_for_postgresql_type_self_where_filter_token_stream>}
                },
                &ident,
                &match &should_add_declaration_of_struct_ident_generic {
                    ShouldAddDeclarationOfStructIdentGeneric::True => &t_annotation_generic_token_stream,
                    ShouldAddDeclarationOfStructIdentGeneric::False => &proc_macro2_token_stream_new
                },
                &where_query_part_content_token_stream,
                &where_query_bind_content_token_stream,
                &postgresql_crud_macros_common::PostgresqlTypeSelfWhereFilterPath::Crate,
            );
            quote::quote! {
                #struct_token_stream
                #maybe_try_new_logic_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                #impl_postgresql_type_self_where_filter_token_stream
            }
        };
        // let filter_array_token_stream = Filter::into_array().map(|element|generate_filters_token_stream(&element));
        let equal_token_stream = generate_filters_token_stream(&Filter::Equal);
        let greater_than_token_stream = generate_filters_token_stream(&Filter::GreaterThan);
        let between_token_stream = generate_filters_token_stream(&Filter::Between);
        let in_token_stream = generate_filters_token_stream(&Filter::In);
        let case_sensitive_regular_expression_token_stream = generate_filters_token_stream(&Filter::CaseSensitiveRegularExpression);
        let case_insensitive_regular_expression_token_stream = generate_filters_token_stream(&Filter::CaseInsensitiveRegularExpression);
        let before_token_stream = generate_filters_token_stream(&Filter::Before);
        let current_date_token_stream = generate_filters_token_stream(&Filter::CurrentDate);
        let greater_than_current_date_token_stream = generate_filters_token_stream(&Filter::GreaterThanCurrentDate);
        let current_timestamp_token_stream = generate_filters_token_stream(&Filter::CurrentTimestamp);
        let greater_than_current_timestamp_token_stream = generate_filters_token_stream(&Filter::GreaterThanCurrentTimestamp);
        let current_time_token_stream = generate_filters_token_stream(&Filter::CurrentTime);
        let greater_than_current_time_token_stream = generate_filters_token_stream(&Filter::GreaterThanCurrentTime);
        let length_more_than_token_stream = generate_filters_token_stream(&Filter::LengthMoreThan);
        let equal_to_encoded_string_representation_token_stream = generate_filters_token_stream(&Filter::EqualToEncodedStringRepresentation);
        let value_is_contained_within_range_token_stream = generate_filters_token_stream(&Filter::ValueIsContainedWithinRange);
        let contains_another_range_token_stream = generate_filters_token_stream(&Filter::ContainsAnotherRange);
        let strictly_to_left_of_range_token_stream = generate_filters_token_stream(&Filter::StrictlyToLeftOfRange);
        let strictly_to_right_of_range_token_stream = generate_filters_token_stream(&Filter::StrictlyToRightOfRange);
        let included_lower_bound_token_stream = generate_filters_token_stream(&Filter::IncludedLowerBound);
        let excluded_upper_bound_token_stream = generate_filters_token_stream(&Filter::ExcludedUpperBound);
        let greater_than_lower_bound_token_stream = generate_filters_token_stream(&Filter::GreaterThanLowerBound);
        let overlap_with_range_token_stream = generate_filters_token_stream(&Filter::OverlapWithRange);
        let adjacent_with_range_token_stream = generate_filters_token_stream(&Filter::AdjacentWithRange);
        let range_length_token_stream = generate_filters_token_stream(&Filter::RangeLength);
        // let _token_stream = generate_filters_token_stream(&Filter::);
        quote::quote! {
            //#(#filter_array_token_stream)*

            // #equal_token_stream
            // #greater_than_token_stream
            // #between_token_stream
            // #in_token_stream
            // #case_sensitive_regular_expression_token_stream
            // #case_insensitive_regular_expression_token_stream
            // #before_token_stream
            // #current_date_token_stream
            // #greater_than_current_date_token_stream
            // #current_timestamp_token_stream
            // #greater_than_current_timestamp_token_stream
            // #current_time_token_stream
            // #greater_than_current_time_token_stream
            // #length_more_than_token_stream
            // #equal_to_encoded_string_representation_token_stream
            // #value_is_contained_within_range_token_stream
            // #contains_another_range_token_stream
            // #strictly_to_left_of_range_token_stream
            // #strictly_to_right_of_range_token_stream
            // #included_lower_bound_token_stream
            // #excluded_upper_bound_token_stream
            // #greater_than_lower_bound_token_stream
            // #overlap_with_range_token_stream
            // #adjacent_with_range_token_stream
            // #range_length_token_stream
            //#_token_stream
        }
    };
    let generated = quote::quote!{
        #postgresql_type_token_stream
        #postgresql_json_type_token_stream
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlTypeWhereElementFilters",
    //     &generated,
    // );
    generated.into()
}