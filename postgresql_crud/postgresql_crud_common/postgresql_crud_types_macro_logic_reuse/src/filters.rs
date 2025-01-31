pub struct Equal;
impl crate::WhereOperatorName for Equal {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::EqualUpperCamelCase
    }
}
impl Equal {
    fn generate_additional_type_declaration_token_stream(type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        quote::quote!{pub #value_snake_case: #type_token_stream}
    }
    fn generate_additional_default_initialization_token_stream(initialization_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        quote::quote!{#value_snake_case: #initialization_token_stream}
    }
    fn generate_try_generate_bind_increments_token_stream(is_nullable_postgresql_type: &crate::IsNullablePostgresqlType) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let match_increment_checked_add_token_stream = {
            let increment_snake_case = naming::IncrementSnakeCase;
            let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
            let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
            quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} = ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            }
        };
        match &is_nullable_postgresql_type {
            crate::IsNullablePostgresqlType::NullablePostgresqlType {
                where_operator_type: _,
            } => {
                quote::quote!{
                    if self.#value_snake_case.is_some() {
                        #match_increment_checked_add_token_stream
                    }
                    else {
                        Ok(format!(
                            "{}({} is null)",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                        ))
                    }
                }
            },
            crate::IsNullablePostgresqlType::NotNullPostgresqlType { where_operator_type: _, } => match_increment_checked_add_token_stream,
            crate::IsNullablePostgresqlType::PostgresqlJsonType => match_increment_checked_add_token_stream,
        }
    }
    fn generate_bind_value_to_query_token_stream(is_nullable_postgresql_type: &crate::IsNullablePostgresqlType) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let generate_query_equals_query_bind_token_stream = |bind_content_token_stream: &proc_macro2::TokenStream|{
            quote::quote!{
                #query_snake_case = #query_snake_case.bind(#bind_content_token_stream);
            }
        };
        let additional_content_token_stream = match &is_nullable_postgresql_type {
            crate::IsNullablePostgresqlType::NullablePostgresqlType {
                where_operator_type,
            } => {
                let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
                quote::quote!{
                    if let Some(#value_snake_case) = self.#value_snake_case {
                        #query_snake_case = #query_snake_case.bind(#value_snake_case #where_operator_type_additional_bind_token_stream);
                    }
                }
            },
            crate::IsNullablePostgresqlType::NotNullPostgresqlType {
                where_operator_type,
            } => generate_query_equals_query_bind_token_stream(&{
                let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
                quote::quote!{self.#value_snake_case #where_operator_type_additional_bind_token_stream}
            }),
            //todo maybe instead of wrapping into sqlx::types::Json - impl Encode? 
            crate::IsNullablePostgresqlType::PostgresqlJsonType => generate_query_equals_query_bind_token_stream(&quote::quote!{sqlx::types::Json(self.#value_snake_case)}),
        };
        quote::quote!{
            #additional_content_token_stream
            #query_snake_case
        }
    }
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
        where_operator_type: &crate::WhereOperatorType,
    ) -> proc_macro2::TokenStream {
        let generate_postgresql_type_ident_where_element_tokens_upper_camel_case = |prefix: &dyn std::fmt::Display|{
            let postfix: &dyn naming::StdFmtDisplayPlusQuoteToTokens = crate::WhereOperatorName::upper_camel_case(self);
            let value = format!("{prefix}{postfix}");
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
        let should_where_element_fields_be_public = crate::ShouldWhereElementFieldsBePublic::True;
        let should_implement_schemars_json_schema = crate::ShouldImplementSchemarsJsonSchema::False;
        match &is_nullable {
            crate::IsNullable::True => {
                let is_nullable_postgresql_type = crate::IsNullablePostgresqlType::NullablePostgresqlType {
                    where_operator_type: &where_operator_type,
                };
                crate::generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
                    &postgresql_type_or_json_type,
                    &generate_postgresql_type_ident_where_element_tokens_upper_camel_case(
                        &naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementUpperCamelCase::from_tokens(&ident)
                    ),
                    should_where_element_fields_be_public,
                    &should_implement_schemars_json_schema,
                    &Self::generate_additional_type_declaration_token_stream(&where_operator_type.std_option_option_type_token_stream()),
                    &Self::generate_additional_default_initialization_token_stream(&where_operator_type.std_option_option_default_initialization_token_stream()),
                    &Self::generate_try_generate_bind_increments_token_stream(&is_nullable_postgresql_type),
                    &Self::generate_bind_value_to_query_token_stream(&is_nullable_postgresql_type),
                )
            },
            crate::IsNullable::False => {
                let is_nullable_postgresql_type = crate::IsNullablePostgresqlType::NotNullPostgresqlType {
                    where_operator_type: &where_operator_type,
                };
                crate::generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
                    &postgresql_type_or_json_type,
                    &generate_postgresql_type_ident_where_element_tokens_upper_camel_case(
                        &naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident)
                    ),
                    should_where_element_fields_be_public,
                    &should_implement_schemars_json_schema,
                    &Self::generate_additional_type_declaration_token_stream(&where_operator_type.type_token_stream()),
                    &Self::generate_additional_default_initialization_token_stream(&where_operator_type.default_initialization_token_stream()),
                    &Self::generate_try_generate_bind_increments_token_stream(&is_nullable_postgresql_type),
                    &Self::generate_bind_value_to_query_token_stream(&is_nullable_postgresql_type),
                )
            }
        }
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        variant: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&variant));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let is_nullable_postgresql_type = crate::IsNullablePostgresqlType::PostgresqlJsonType;
        let (
            postgresql_json_type_handle,
            postgresql_json_type_pattern
        ) = variant.to_postgresql_json_type_handle_and_postgresql_json_type_pattern();
        crate::generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &crate::PostgresqlTypeOrJsonType::PostgresqlJsonType,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::True,
            &crate::ShouldImplementSchemarsJsonSchema::True,
            &Self::generate_additional_type_declaration_token_stream(&postgresql_json_type_pattern.field_type(&postgresql_json_type_handle)),
            &Self::generate_additional_default_initialization_token_stream(&postgresql_json_type_pattern.initialization_token_stream()),
            &Self::generate_try_generate_bind_increments_token_stream(&is_nullable_postgresql_type),
            &Self::generate_bind_value_to_query_token_stream(&is_nullable_postgresql_type),
        )
    }
}

pub struct GreaterThan;
impl crate::WhereOperatorName for GreaterThan {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::GreaterThanUpperCamelCase
    }
}
impl GreaterThan {
    fn generate_additional_type_declaration_token_stream(type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        quote::quote!{pub #value_snake_case: #type_token_stream}
    }
    fn generate_additional_default_initialization_token_stream(initialization_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        quote::quote!{#value_snake_case: #initialization_token_stream}
    }
    fn generate_try_generate_bind_increments_token_stream() -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        quote::quote!{
            match #increment_snake_case.checked_add(1) {
                Some(#value_snake_case) => {
                    *#increment_snake_case = #value_snake_case;
                    Ok(format!(
                        "{}({} > ${})",
                        &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                        #column_snake_case,
                        #increment_snake_case
                    ))
                },
                None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            }
        }
    }
    fn generate_bind_value_to_query_token_stream(bind_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let query_snake_case = naming::QuerySnakeCase;
        quote::quote!{
            #query_snake_case = #query_snake_case.bind(#bind_token_stream);
            #query_snake_case
        }
    }
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
        where_operator_type: &crate::WhereOperatorType,
    ) -> proc_macro2::TokenStream {
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &Self::generate_additional_type_declaration_token_stream(&where_operator_type.type_token_stream()),
            &Self::generate_additional_default_initialization_token_stream(&where_operator_type.default_initialization_token_stream()),
            &Self::generate_try_generate_bind_increments_token_stream(),
            &Self::generate_bind_value_to_query_token_stream(&{
                let value_snake_case = naming::ValueSnakeCase;
                let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
                quote::quote!{self.#value_snake_case #where_operator_type_additional_bind_token_stream}
            }),
        )
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        variant: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&variant));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let (
            postgresql_json_type_handle,
            postgresql_json_type_pattern
        ) = variant.to_postgresql_json_type_handle_and_postgresql_json_type_pattern();
        crate::generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &crate::PostgresqlTypeOrJsonType::PostgresqlJsonType,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::True,
            &crate::ShouldImplementSchemarsJsonSchema::True,
            &Self::generate_additional_type_declaration_token_stream(&postgresql_json_type_pattern.field_type(&postgresql_json_type_handle)),
            &Self::generate_additional_default_initialization_token_stream(&postgresql_json_type_pattern.initialization_token_stream()),
            &Self::generate_try_generate_bind_increments_token_stream(),
            &Self::generate_bind_value_to_query_token_stream(&{
                let value_snake_case = naming::ValueSnakeCase;
                quote::quote!{sqlx::types::Json(self.#value_snake_case)}
            }),
        )
    }
}

pub enum BetweenTryNewErrorType {
    StartMoreOrEqualToEnd,
    StartIsEqualToEnd
}
impl BetweenTryNewErrorType {
    fn try_new_error_named_upper_camel_case_token_stream(&self) -> &'static dyn quote::ToTokens {
        match self {
            Self::StartMoreOrEqualToEnd => &naming::StartMoreOrEqualToEndUpperCamelCase,
            Self::StartIsEqualToEnd => &naming::StartIsEqualToEndUpperCamelCase,
        }
    }
    fn try_new_error_named_compare_symbol_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::StartMoreOrEqualToEnd => quote::quote!{<},
            Self::StartIsEqualToEnd => quote::quote!{!=},
        }
    }
}
pub enum ShouldAddDotZero {
    True,
    False,
}
impl quote::ToTokens for ShouldAddDotZero {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote!{.0}.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}
//todo fix for json type Between Some(0) and Some(0) -> remove options
pub struct Between;
impl crate::WhereOperatorName for Between {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::BetweenUpperCamelCase
    }
}
impl Between {
    fn generate_try_new_error_named_variants_token_stream(
        between_try_new_error_type: &BetweenTryNewErrorType,
        type_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        let start_snake_case = naming::StartSnakeCase;
        let end_snake_case = naming::EndSnakeCase;
        let try_new_error_named_upper_camel_case_token_stream = &between_try_new_error_type.try_new_error_named_upper_camel_case_token_stream();
        quote::quote!{
            #try_new_error_named_upper_camel_case_token_stream {
                #[eo_to_std_string_string_serialize_deserialize]
                #start_snake_case: #type_token_stream,
                #[eo_to_std_string_string_serialize_deserialize]
                #end_snake_case: #type_token_stream,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
            }
        }
    }
    fn generate_try_new_content_token_stream(
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
        between_try_new_error_type: &BetweenTryNewErrorType,
        start_end_additional_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        let start_snake_case = naming::StartSnakeCase;
        let end_snake_case = naming::EndSnakeCase;
        let try_new_error_named_compare_symbol_token_stream = between_try_new_error_type.try_new_error_named_compare_symbol_token_stream();
        let try_new_error_named_upper_camel_case_token_stream = between_try_new_error_type.try_new_error_named_upper_camel_case_token_stream();
        let postgresql_type_or_json_type_ident_where_element_between_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match &postgresql_type_or_json_type {
            crate::PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementBetweenTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementBetweenTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        };
        quote::quote!{
            if 
                #start_snake_case
                #start_end_additional_token_stream
                #try_new_error_named_compare_symbol_token_stream
                #end_snake_case
                #start_end_additional_token_stream
            {
                Ok(Self {
                    logical_operator,
                    #start_snake_case,
                    #end_snake_case
                })
            }
            else {
                Err(#postgresql_type_or_json_type_ident_where_element_between_try_new_error_named_upper_camel_case::#try_new_error_named_upper_camel_case_token_stream {
                    #start_snake_case,
                    #end_snake_case,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            }
        }
    }
    fn generate_impl_deserialize_token_stream(
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
        self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
        element_type_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        let postgresql_type_or_json_type_ident_where_element_between_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_or_json_type {
            crate::PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementBetweenUpperCamelCase::from_tokens(&ident),
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementBetweenUpperCamelCase::from_tokens(&ident),
        };
        let (
            struct_postgresql_type_or_json_type_ident_where_element_between_double_quotes_token_stream,
            struct_postgresql_type_or_json_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
            postgresql_type_or_json_type_ident_where_element_between_double_quotes_token_stream
        ) = crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_element_between_upper_camel_case, 3, &self_upper_camel_case);
        quote::quote! {
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for #postgresql_type_or_json_type_ident_where_element_between_upper_camel_case {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
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
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
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
                                    "logical_operator" => _serde::__private::Ok(__Field::__field0),
                                    "start" => _serde::__private::Ok(__Field::__field1),
                                    "end" => _serde::__private::Ok(__Field::__field2),
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
                                    b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                                    b"start" => _serde::__private::Ok(__Field::__field1),
                                    b"end" => _serde::__private::Ok(__Field::__field2),
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
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<
                                #postgresql_type_or_json_type_ident_where_element_between_upper_camel_case,
                            >,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #postgresql_type_or_json_type_ident_where_element_between_upper_camel_case;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter<'_>,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    #struct_postgresql_type_or_json_type_ident_where_element_between_double_quotes_token_stream,
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
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    crate::LogicalOperator,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &#struct_postgresql_type_or_json_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    #element_type_token_stream,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &#struct_postgresql_type_or_json_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    #element_type_token_stream,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &#struct_postgresql_type_or_json_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                match #postgresql_type_or_json_type_ident_where_element_between_upper_camel_case::try_new(__field0, __field1, __field2) {
                                    Ok(value) => _serde::__private::Ok(value),
                                    Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                }
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<
                                    crate::LogicalOperator,
                                > = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<#element_type_token_stream> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<#element_type_token_stream> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "logical_operator",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    crate::LogicalOperator,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    #element_type_token_stream,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    #element_type_token_stream,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("logical_operator")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("start")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("end")?
                                    }
                                };
                                match #postgresql_type_or_json_type_ident_where_element_between_upper_camel_case::try_new(__field0, __field1, __field2) {
                                    Ok(value) => _serde::__private::Ok(value),
                                    Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                }
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "logical_operator",
                            "start",
                            "end",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #postgresql_type_or_json_type_ident_where_element_between_double_quotes_token_stream,
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    #postgresql_type_or_json_type_ident_where_element_between_upper_camel_case,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
        }
    }
    fn generate_additional_type_declaration_token_stream(type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let start_snake_case = naming::StartSnakeCase;
        let end_snake_case = naming::EndSnakeCase;
        quote::quote!{
            #start_snake_case: #type_token_stream,
            #end_snake_case: #type_token_stream
        }
    }
    fn generate_additional_default_initialization_token_stream(initialization_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let start_snake_case = naming::StartSnakeCase;
        let end_snake_case = naming::EndSnakeCase;
        quote::quote!{
            #start_snake_case: #initialization_token_stream,
            #end_snake_case: #initialization_token_stream,
        }
    }
    fn generate_try_generate_bind_increments_token_stream() -> proc_macro2::TokenStream {
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        quote::quote!{
            match #increment_snake_case.checked_add(1) {
                Some(first_value) => {
                    *#increment_snake_case = first_value;
                    match #increment_snake_case.checked_add(1) {
                        Some(second_value) => {
                            *#increment_snake_case = second_value;
                            let between_snake_case = naming::BetweenSnakeCase;
                            let and_snake_case = naming::AndSnakeCase;
                            Ok(format!("{}({column} {between_snake_case} ${first_value} {and_snake_case} ${second_value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                        },
                        None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                },
                None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            }
        }
    }
    fn generate_bind_value_to_query_token_stream(
        start_bind_token_stream: &dyn quote::ToTokens,
        end_bind_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        let query_snake_case = naming::QuerySnakeCase;
        quote::quote!{
            #query_snake_case = #query_snake_case.bind(#start_bind_token_stream);
            #query_snake_case = #query_snake_case.bind(#end_bind_token_stream);
            #query_snake_case
        }
    }
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
        where_operator_type: &crate::WhereOperatorType,
        between_try_new_error_type: &BetweenTryNewErrorType,
        should_add_dot_zero: &ShouldAddDotZero,
    ) -> proc_macro2::TokenStream {
        let where_operator_type_type_token_stream = where_operator_type.type_token_stream();
        let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &self_upper_camel_case,
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &ident,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &Self::generate_try_new_error_named_variants_token_stream(
                    &between_try_new_error_type,
                    &where_operator_type_type_token_stream,
                ),
                try_new_additional_input_parameters_token_stream: &Self::generate_additional_type_declaration_token_stream(&where_operator_type_type_token_stream),
                try_new_content_token_stream: &Self::generate_try_new_content_token_stream(
                    &ident,
                    &postgresql_type_or_json_type,
                    &between_try_new_error_type,
                    &quote::quote!{#where_operator_type_additional_bind_token_stream #should_add_dot_zero},
                ),
                impl_deserialize_token_stream: &Self::generate_impl_deserialize_token_stream(
                    &ident,
                    &postgresql_type_or_json_type,
                    &self_upper_camel_case,
                    &where_operator_type_type_token_stream,
                ),
            },
            &Self::generate_additional_type_declaration_token_stream(&where_operator_type_type_token_stream),
            &Self::generate_additional_default_initialization_token_stream(&where_operator_type.default_initialization_token_stream()),
            &Self::generate_try_generate_bind_increments_token_stream(),
            &{
                let start_snake_case = naming::StartSnakeCase;
                let end_snake_case = naming::EndSnakeCase;
                Self::generate_bind_value_to_query_token_stream(
                    &quote::quote!{self.#start_snake_case #where_operator_type_additional_bind_token_stream},
                    &quote::quote!{self.#end_snake_case #where_operator_type_additional_bind_token_stream},
                )
            }
        )
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        between_try_new_error_type: &BetweenTryNewErrorType,
        variant: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&variant));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
        let non_optional_field_type = {
            let (
                postgresql_json_type_handle,
                postgresql_json_type_pattern
            ) = variant.to_postgresql_json_type_handle_and_postgresql_json_type_pattern();
            postgresql_json_type_pattern.non_optional_field_type(&postgresql_json_type_handle)
        };
        let additional_type_declaration_token_stream = Self::generate_additional_type_declaration_token_stream(&non_optional_field_type);
        crate::generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &postgresql_type_or_json_type,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &variant,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &Self::generate_try_new_error_named_variants_token_stream(
                    &between_try_new_error_type,
                    &non_optional_field_type,
                ),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &Self::generate_try_new_content_token_stream(
                    &variant,
                    &postgresql_type_or_json_type,
                    &between_try_new_error_type,
                    &proc_macro2::TokenStream::new(),
                ),
                impl_deserialize_token_stream: &Self::generate_impl_deserialize_token_stream(
                    &variant,
                    &postgresql_type_or_json_type,
                    &self_upper_camel_case,
                    &non_optional_field_type,
                ),
            },
            &crate::ShouldImplementSchemarsJsonSchema::True,
            &additional_type_declaration_token_stream,
            &Self::generate_additional_default_initialization_token_stream(&crate::PostgresqlJsonTypePattern::from(variant).non_optional_initialization_token_stream()),
            &Self::generate_try_generate_bind_increments_token_stream(),
            &{
                let start_snake_case = naming::StartSnakeCase;
                let end_snake_case = naming::EndSnakeCase;
                Self::generate_bind_value_to_query_token_stream(
                    &quote::quote!{sqlx::types::Json(self.#start_snake_case)},
                    &quote::quote!{sqlx::types::Json(self.#end_snake_case)},
                )
            }
        )
    }
}

pub struct In;
impl crate::WhereOperatorName for In {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::InUpperCamelCase
    }
}
impl In {
    fn generate_try_new_error_named_variants_token_stream(not_unique_value_type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let is_empty_upper_camel_case = naming::IsEmptyUpperCamelCase;
        let not_unique_upper_camel_case = naming::NotUniqueUpperCamelCase;
        quote::quote!{
            #is_empty_upper_camel_case {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
            },
            #not_unique_upper_camel_case {
                #[eo_to_std_string_string_serialize_deserialize]
                #value_snake_case: #not_unique_value_type_token_stream,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
            },
        }
    }
    fn generate_try_new_content_token_stream(
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    ) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let is_empty_upper_camel_case = naming::IsEmptyUpperCamelCase;
        let not_unique_upper_camel_case = naming::NotUniqueUpperCamelCase;
        let element_snake_case = naming::ElementSnakeCase;
        let acc_snake_case = naming::AccSnakeCase;
        let postgresql_type_or_json_type_ident_where_element_in_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match &postgresql_type_or_json_type {
            crate::PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementInTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementInTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        };
        quote::quote!{
            if #value_snake_case.is_empty() {
                return Err(#postgresql_type_or_json_type_ident_where_element_in_try_new_error_named_upper_camel_case::#is_empty_upper_camel_case {
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            {
                let mut #acc_snake_case = vec![];
                for #element_snake_case in &#value_snake_case {
                    if !#acc_snake_case.contains(&#element_snake_case) {
                        #acc_snake_case.push(#element_snake_case);
                    } else {
                        return Err(#postgresql_type_or_json_type_ident_where_element_in_try_new_error_named_upper_camel_case::#not_unique_upper_camel_case {
                            #value_snake_case: #element_snake_case.clone(),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            Ok(Self{
                logical_operator,
                #value_snake_case
            })
        }
    }
    fn generate_impl_deserialize_token_stream(
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
        self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
        vec_type_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        let postgresql_type_or_json_type_ident_where_element_in_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_or_json_type {
            crate::PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementInUpperCamelCase::from_tokens(&ident),
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementInUpperCamelCase::from_tokens(&ident),
        };
        let (
            struct_postgresql_type_or_json_type_ident_where_element_in_double_quotes_token_stream,
            struct_postgresql_type_or_json_type_ident_where_element_in_with_2_elements_double_quotes_token_stream,
            postgresql_type_or_json_type_ident_where_element_in_double_quotes_token_stream
        ) = crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_element_in_upper_camel_case, 2, &self_upper_camel_case);
        quote::quote! {
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for #postgresql_type_or_json_type_ident_where_element_in_upper_camel_case {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
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
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
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
                                    "logical_operator" => _serde::__private::Ok(__Field::__field0),
                                    "value" => _serde::__private::Ok(__Field::__field1),
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
                                    b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                                    b"value" => _serde::__private::Ok(__Field::__field1),
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
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<
                                #postgresql_type_or_json_type_ident_where_element_in_upper_camel_case,
                            >,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #postgresql_type_or_json_type_ident_where_element_in_upper_camel_case;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter<'_>,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    #struct_postgresql_type_or_json_type_ident_where_element_in_double_quotes_token_stream,
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
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    crate::LogicalOperator,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &#struct_postgresql_type_or_json_type_ident_where_element_in_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    std::vec::Vec<#vec_type_token_stream>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &#struct_postgresql_type_or_json_type_ident_where_element_in_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                match #postgresql_type_or_json_type_ident_where_element_in_upper_camel_case::try_new(__field0, __field1) {
                                    Ok(value) => _serde::__private::Ok(value),
                                    Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                }
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<
                                    crate::LogicalOperator,
                                > = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    std::vec::Vec<#vec_type_token_stream>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "logical_operator",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    crate::LogicalOperator,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    std::vec::Vec<#vec_type_token_stream>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("logical_operator")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("value")?
                                    }
                                };
                                match #postgresql_type_or_json_type_ident_where_element_in_upper_camel_case::try_new(__field0, __field1) {
                                    Ok(value) => _serde::__private::Ok(value),
                                    Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                }
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #postgresql_type_or_json_type_ident_where_element_in_double_quotes_token_stream,
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    #postgresql_type_or_json_type_ident_where_element_in_upper_camel_case,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
        }
    }
    fn generate_additional_type_declaration_token_stream(vec_type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        quote::quote!{
            #value_snake_case: std::vec::Vec<#vec_type_token_stream>
        }
    }
    fn generate_additional_default_initialization_token_stream(default_initialization_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        quote::quote!{
            #value_snake_case: vec![#default_initialization_token_stream]
        }
    }
    fn generate_try_generate_bind_increments_token_stream() -> proc_macro2::TokenStream {
        let element_snake_case = naming::ElementSnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let acc_snake_case = naming::AccSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        quote::quote!{
            let mut #acc_snake_case = std::string::String::default();
            for #element_snake_case in &self.#value_snake_case {
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        #acc_snake_case.push_str(&format!("${},", #value_snake_case));
                    },
                    None => {
                        return Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            let _ = #acc_snake_case.pop();
            let in_snake_case = naming::InSnakeCase;
            Ok(format!(
                "{}({} {in_snake_case} ({}))",
                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                #column_snake_case,
                #acc_snake_case
            ))
        }
    }
    fn generate_bind_value_to_query_token_stream(element_bind_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        let element_snake_case = naming::ElementSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        quote::quote!{
            for #element_snake_case in self.#value_snake_case {
                #query_snake_case = #query_snake_case.bind(#element_bind_token_stream);
            }
            #query_snake_case
        }
    }
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
        where_operator_type: &crate::WhereOperatorType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let where_operator_type_type_token_stream = where_operator_type.type_token_stream();
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &self_upper_camel_case,
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &ident,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &Self::generate_try_new_error_named_variants_token_stream(&where_operator_type_type_token_stream),
                try_new_additional_input_parameters_token_stream: &Self::generate_additional_type_declaration_token_stream(&where_operator_type_type_token_stream),
                try_new_content_token_stream: &Self::generate_try_new_content_token_stream(
                    &ident,
                    &postgresql_type_or_json_type,
                ),
                impl_deserialize_token_stream: &Self::generate_impl_deserialize_token_stream(
                    &ident,
                    &postgresql_type_or_json_type,
                    &self_upper_camel_case,
                    &where_operator_type_type_token_stream,
                ),
            },
            &Self::generate_additional_type_declaration_token_stream(&where_operator_type_type_token_stream),
            &Self::generate_additional_default_initialization_token_stream(&where_operator_type.default_initialization_token_stream()),
            &Self::generate_try_generate_bind_increments_token_stream(),
            &Self::generate_bind_value_to_query_token_stream(&{
                let element_snake_case = naming::ElementSnakeCase;
                let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
                quote::quote!{#element_snake_case #where_operator_type_additional_bind_token_stream}
            }),
        )
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        variant: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        let (
            postgresql_json_type_handle,
            postgresql_json_type_pattern
        ) = variant.to_postgresql_json_type_handle_and_postgresql_json_type_pattern();
        let field_type = postgresql_json_type_pattern.field_type(&postgresql_json_type_handle);

        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&variant));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let non_optional_field_type = postgresql_json_type_pattern.non_optional_field_type(&postgresql_json_type_handle);
        let additional_type_declaration_token_stream = Self::generate_additional_type_declaration_token_stream(&non_optional_field_type);
        crate::generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &postgresql_type_or_json_type,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &variant,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &Self::generate_try_new_error_named_variants_token_stream(&non_optional_field_type),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &Self::generate_try_new_content_token_stream(
                    &variant,
                    &postgresql_type_or_json_type,
                ),
                impl_deserialize_token_stream: &Self::generate_impl_deserialize_token_stream(
                    &variant,
                    &postgresql_type_or_json_type,
                    &self_upper_camel_case,
                    &non_optional_field_type,
                ),
            },
            &crate::ShouldImplementSchemarsJsonSchema::True,
            &additional_type_declaration_token_stream,
            &Self::generate_additional_default_initialization_token_stream(&postgresql_json_type_pattern.non_optional_initialization_token_stream()),
            &Self::generate_try_generate_bind_increments_token_stream(),
            &Self::generate_bind_value_to_query_token_stream(&{
                let element_snake_case = naming::ElementSnakeCase;
                quote::quote!{sqlx::types::Json(#element_snake_case)}
            }),
        )
    }
}

enum RegularExpression {
    CaseSensitive,
    CaseInsensitive
}
impl RegularExpression {
    fn stringified(&self) -> &'static str {
        match &self {
            RegularExpression::CaseSensitive => "",
            RegularExpression::CaseInsensitive => "*",
        }
    }
}

fn generate_regular_expression_try_new_error_named_variants_token_stream() -> proc_macro2::TokenStream {
    let is_empty_upper_camel_case = naming::IsEmptyUpperCamelCase;
    quote::quote!{
        #is_empty_upper_camel_case {
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        }
    }
}
fn generate_regular_expression_try_new_content_token_stream(
    ident: &dyn quote::ToTokens,
    regular_expression: &RegularExpression,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
) -> proc_macro2::TokenStream {
    let is_empty_upper_camel_case = naming::IsEmptyUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    let postgresql_type_ident_where_element_regular_expression_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match &(regular_expression,postgresql_type_or_json_type) {
        (
            RegularExpression::CaseSensitive,
            crate::PostgresqlTypeOrJsonType::PostgresqlType,
        ) => &naming::parameter::PostgresqlTypeSelfWhereElementCaseSensitiveRegularExpressionTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        (
            RegularExpression::CaseInsensitive,
            crate::PostgresqlTypeOrJsonType::PostgresqlType,
        ) => &naming::parameter::PostgresqlTypeSelfWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        (
            RegularExpression::CaseSensitive,
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType,
        ) => &naming::parameter::PostgresqlJsonTypeSelfWhereElementCaseSensitiveRegularExpressionTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        (
            RegularExpression::CaseInsensitive,
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType,
        ) => &naming::parameter::PostgresqlJsonTypeSelfWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
    };
    quote::quote!{
        if !#value_snake_case.is_empty() {
            Ok(Self {
                logical_operator,
                #value_snake_case,
            })
        }
        else {
            Err(#postgresql_type_ident_where_element_regular_expression_try_new_error_named_upper_camel_case::#is_empty_upper_camel_case {
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
fn generate_regular_expression_impl_deserialize_token_stream(
    ident: &dyn quote::ToTokens,
    regular_expression: &RegularExpression,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type_ident_where_element_regular_expression_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &(regular_expression,postgresql_type_or_json_type) {
        (
            RegularExpression::CaseSensitive,
            crate::PostgresqlTypeOrJsonType::PostgresqlType,
        ) => &naming::parameter::PostgresqlTypeSelfWhereElementCaseSensitiveRegularExpressionUpperCamelCase::from_tokens(&ident),
        (
            RegularExpression::CaseInsensitive,
            crate::PostgresqlTypeOrJsonType::PostgresqlType,
        ) => &naming::parameter::PostgresqlTypeSelfWhereElementCaseInsensitiveRegularExpressionUpperCamelCase::from_tokens(&ident),
        (
            RegularExpression::CaseSensitive,
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType,
        ) => &naming::parameter::PostgresqlJsonTypeSelfWhereElementCaseSensitiveRegularExpressionUpperCamelCase::from_tokens(&ident),
        (
            RegularExpression::CaseInsensitive,
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType,
        ) => &naming::parameter::PostgresqlJsonTypeSelfWhereElementCaseInsensitiveRegularExpressionUpperCamelCase::from_tokens(&ident),
    };
    let (
        struct_postgresql_type_or_json_type_ident_where_element_regular_expression_double_quotes_token_stream,
        struct_postgresql_type_or_json_type_ident_where_element_regular_expression_with_2_elements_double_quotes_token_stream,
        postgresql_type_or_json_type_ident_where_element_regular_expression_double_quotes_token_stream
    ) = crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_element_regular_expression_upper_camel_case, 2, &self_upper_camel_case);
    let std_string_string = token_patterns::StdStringString;
    quote::quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for #postgresql_type_or_json_type_ident_where_element_regular_expression_upper_camel_case {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
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
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
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
                                "logical_operator" => _serde::__private::Ok(__Field::__field0),
                                "value" => _serde::__private::Ok(__Field::__field1),
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
                                b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                                b"value" => _serde::__private::Ok(__Field::__field1),
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
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            #postgresql_type_or_json_type_ident_where_element_regular_expression_upper_camel_case,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type_or_json_type_ident_where_element_regular_expression_upper_camel_case;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                #struct_postgresql_type_or_json_type_ident_where_element_regular_expression_double_quotes_token_stream,
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
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                crate::LogicalOperator,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &#struct_postgresql_type_or_json_type_ident_where_element_regular_expression_with_2_elements_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                #std_string_string,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &#struct_postgresql_type_or_json_type_ident_where_element_regular_expression_with_2_elements_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            match #postgresql_type_or_json_type_ident_where_element_regular_expression_upper_camel_case::try_new(__field0, __field1) {
                                Ok(value) => _serde::__private::Ok(value),
                                Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                            }
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                crate::LogicalOperator,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<#std_string_string> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "logical_operator",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::LogicalOperator,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                #std_string_string,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("logical_operator")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("value")?
                                }
                            };
                            match #postgresql_type_or_json_type_ident_where_element_regular_expression_upper_camel_case::try_new(__field0, __field1) {
                                Ok(value) => _serde::__private::Ok(value),
                                Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                            }
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        #postgresql_type_or_json_type_ident_where_element_regular_expression_double_quotes_token_stream,
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                #postgresql_type_or_json_type_ident_where_element_regular_expression_upper_camel_case,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
}
fn generate_regular_expression_additional_type_declaration_token_stream() -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let std_string_string = token_patterns::StdStringString;
    quote::quote!{#value_snake_case: #std_string_string}
}
fn generate_regular_expression_additional_default_initialization_token_stream() -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
    quote::quote!{#value_snake_case: #core_default_default_default}
}
fn generate_regular_expression_postgresql_type_self_where_try_generate_bind_increments_token_stream(
    regular_expression: &RegularExpression,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let case_stringified = regular_expression.stringified();
    //using postgresql operator -> than converting it into text returns original string wrapped in double quotes ("original string" instead of original string). it causes problems with regular expression api.
    //this code remove double quotes from start and end of string(text) to make regular expression correct
    //decided to use -> postgresql operator instead of ->> operation(cast to text) coz cast to text also affect other types like bool and number. they just will fail at decoding
    let path = match &postgresql_type_or_json_type {
        crate::PostgresqlTypeOrJsonType::PostgresqlType => std::string::String::from("{}"),
        crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => std::string::String::from("substring(({})::text from 2 for length(({})::text) - 2)"),
    };
    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({path} ~{case_stringified} ${{}})"));
    let column_or_columns_token_stream = match &postgresql_type_or_json_type {
        crate::PostgresqlTypeOrJsonType::PostgresqlType => quote::quote!{#column_snake_case,},
        crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => quote::quote!{#column_snake_case,#column_snake_case,},
    };
    quote::quote!{
        match #increment_snake_case.checked_add(1) {
            Some(#value_snake_case) => {
                *#increment_snake_case = #value_snake_case;
                Ok(format!(
                    #format_handle_token_stream,
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_or_columns_token_stream
                    #increment_snake_case
                ))
            },
            None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
fn generate_regular_expression_postgresql_type_self_where_bind_value_to_query_token_stream() -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    quote::quote!{
        #query_snake_case = #query_snake_case.bind(self.#value_snake_case);
        #query_snake_case
    }
}
fn generate_regular_expression_postgresql_type_tokens_where_element_variant_handle_token_stream(
    ident: &dyn quote::ToTokens,
    is_nullable: &crate::IsNullable,
    regular_expression: &RegularExpression,
    self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
    crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
        &ident,
        &self_upper_camel_case,
        &is_nullable,
        crate::ShouldWhereElementFieldsBePublic::False {
            ident: &ident,
            postfix: &self_upper_camel_case,
            try_new_error_named_variants_token_stream: &generate_regular_expression_try_new_error_named_variants_token_stream(),
            try_new_additional_input_parameters_token_stream: &generate_regular_expression_additional_type_declaration_token_stream(),
            try_new_content_token_stream: &generate_regular_expression_try_new_content_token_stream(
                &ident,
                &regular_expression,
                &postgresql_type_or_json_type,
            ),
            impl_deserialize_token_stream: &generate_regular_expression_impl_deserialize_token_stream(
                &ident,
                &regular_expression,
                &postgresql_type_or_json_type,
                &self_upper_camel_case,
            ),
        },
        &generate_regular_expression_additional_type_declaration_token_stream(),
        &generate_regular_expression_additional_default_initialization_token_stream(),
        &generate_regular_expression_postgresql_type_self_where_try_generate_bind_increments_token_stream(&regular_expression, &postgresql_type_or_json_type),
        &generate_regular_expression_postgresql_type_self_where_bind_value_to_query_token_stream()
    )
}
fn generate_regular_expression_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
    ident: &dyn quote::ToTokens,
    regular_expression: &RegularExpression,
    self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
        let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&ident));
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
    crate::generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
        &crate::PostgresqlTypeOrJsonType::PostgresqlJsonType,
        &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
        crate::ShouldWhereElementFieldsBePublic::False {
            ident: &ident,
            postfix: &self_upper_camel_case,
            try_new_error_named_variants_token_stream: &generate_regular_expression_try_new_error_named_variants_token_stream(),
            try_new_additional_input_parameters_token_stream: &generate_regular_expression_additional_type_declaration_token_stream(),
            try_new_content_token_stream: &generate_regular_expression_try_new_content_token_stream(
                &ident,
                &regular_expression,
                &postgresql_type_or_json_type,
            ),
            impl_deserialize_token_stream: &generate_regular_expression_impl_deserialize_token_stream(
                &ident,
                &regular_expression,
                &postgresql_type_or_json_type,
                &self_upper_camel_case,
            ),
        },
        &crate::ShouldImplementSchemarsJsonSchema::True,
        &generate_regular_expression_additional_type_declaration_token_stream(),
        &generate_regular_expression_additional_default_initialization_token_stream(),
        &generate_regular_expression_postgresql_type_self_where_try_generate_bind_increments_token_stream(&regular_expression, &postgresql_type_or_json_type),
        &generate_regular_expression_postgresql_type_self_where_bind_value_to_query_token_stream()
    )
}
pub struct CaseSensitiveRegularExpression;
impl crate::WhereOperatorName for CaseSensitiveRegularExpression {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::CaseSensitiveRegularExpressionUpperCamelCase
    }
}
impl CaseSensitiveRegularExpression {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        generate_regular_expression_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &RegularExpression::CaseSensitive,
            crate::WhereOperatorName::upper_camel_case(self),
        )
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        generate_regular_expression_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &RegularExpression::CaseSensitive,
            crate::WhereOperatorName::upper_camel_case(self),
        )
    }
}
pub struct CaseInsensitiveRegularExpression;
impl crate::WhereOperatorName for CaseInsensitiveRegularExpression {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::CaseInsensitiveRegularExpressionUpperCamelCase
    }
}
impl CaseInsensitiveRegularExpression {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        generate_regular_expression_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &RegularExpression::CaseInsensitive,
            crate::WhereOperatorName::upper_camel_case(self),
        )
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        generate_regular_expression_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &RegularExpression::CaseInsensitive,
            crate::WhereOperatorName::upper_camel_case(self),
        )
    }
}

pub struct Before;
impl crate::WhereOperatorName for Before {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::BeforeUpperCamelCase
    }
}
impl Before {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let query_snake_case = naming::QuerySnakeCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} < ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct CurrentDate;
impl crate::WhereOperatorName for CurrentDate {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::CurrentDateUpperCamelCase
    }
}
impl CurrentDate {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} = current_date)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &naming::QuerySnakeCase
        )
    }
}

pub struct GreaterThanCurrentDate;
impl crate::WhereOperatorName for GreaterThanCurrentDate {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::GreaterThanCurrentDateUpperCamelCase
    }
}
impl GreaterThanCurrentDate {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} > current_date)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &naming::QuerySnakeCase
        )
    }
}

pub struct CurrentTimestamp;
impl crate::WhereOperatorName for CurrentTimestamp {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::CurrentTimestampUpperCamelCase
    }
}
impl CurrentTimestamp {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} = current_timestamp)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &naming::QuerySnakeCase
        )
    }
}

pub struct GreaterThanCurrentTimestamp;
impl crate::WhereOperatorName for GreaterThanCurrentTimestamp {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::GreaterThanCurrentTimestampUpperCamelCase
    }
}
impl GreaterThanCurrentTimestamp {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} > current_timestamp)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &naming::QuerySnakeCase
        )
    }
}

pub struct CurrentTime;
impl crate::WhereOperatorName for CurrentTime {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::CurrentTimeUpperCamelCase
    }
}
impl CurrentTime {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} = current_time)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &naming::QuerySnakeCase
        )
    }
}

pub struct GreaterThanCurrentTime;
impl crate::WhereOperatorName for GreaterThanCurrentTime {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::GreaterThanCurrentTimeUpperCamelCase
    }
}
impl GreaterThanCurrentTime {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} > current_time)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &naming::QuerySnakeCase
        )
    }
}

pub struct LengthMoreThan;
impl crate::WhereOperatorName for LengthMoreThan {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::LengthMoreThanUpperCamelCase
    }
}
impl LengthMoreThan {
    fn length_is_negative_upper_camel_case() -> naming::LengthIsNegativeUpperCamelCase {
        naming::LengthIsNegativeUpperCamelCase
    }
    fn length_is_negative_snake_case() -> naming::LengthIsNegativeSnakeCase {
        naming::LengthIsNegativeSnakeCase
    }
    fn length_more_than_snake_case() -> naming::LengthMoreThanSnakeCase {
        naming::LengthMoreThanSnakeCase
    }
    fn std_primitive_i64_token_stream() -> proc_macro2::TokenStream {
        quote::quote!{std::primitive::i64}
    }
    fn generate_try_new_error_named_variants_token_stream() -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let length_is_negative_upper_camel_case = Self::length_is_negative_upper_camel_case();
        let std_primitive_i64_token_stream = Self::std_primitive_i64_token_stream();
        quote::quote!{
            #length_is_negative_upper_camel_case {
                #[eo_to_std_string_string_serialize_deserialize]
                #value_snake_case: #std_primitive_i64_token_stream,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
            },
        }
    }
    fn generate_try_new_content_token_stream(
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    ) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let postgresql_type_or_json_type_ident_where_element_length_more_than_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match &postgresql_type_or_json_type {
            crate::PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementLengthMoreThanTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementLengthMoreThanTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        };
        let length_more_than_snake_case = Self::length_more_than_snake_case();
        let length_is_negative_upper_camel_case = Self::length_is_negative_upper_camel_case();
        quote::quote!{
            if #length_more_than_snake_case > -1 {
                Ok(Self{
                    logical_operator,
                    #length_more_than_snake_case
                })
            }
            else {
                Err(#postgresql_type_or_json_type_ident_where_element_length_more_than_try_new_error_named_upper_camel_case::#length_is_negative_upper_camel_case {
                    #value_snake_case: #length_more_than_snake_case,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            }
        }
    }
    fn generate_impl_deserialize_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    ) -> proc_macro2::TokenStream {
        let postgresql_type_or_json_type_ident_where_element_length_more_than_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_or_json_type {
            crate::PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementLengthMoreThanUpperCamelCase::from_tokens(&ident),
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementLengthMoreThanUpperCamelCase::from_tokens(&ident),
        };
        let (
            struct_postgresql_type_or_json_type_ident_where_element_length_more_than_double_quotes_token_stream,
            struct_postgresql_type_or_json_type_ident_where_element_length_more_than_with_2_elements_double_quotes_token_stream,
            postgresql_type_or_json_type_ident_where_element_length_more_than_double_quotes_token_stream
        ) = crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_element_length_more_than_upper_camel_case, 2, &crate::WhereOperatorName::upper_camel_case(self));
        let std_primitive_i64_token_stream = Self::std_primitive_i64_token_stream();
        quote::quote! {
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for #postgresql_type_or_json_type_ident_where_element_length_more_than_upper_camel_case {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
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
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
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
                                    "logical_operator" => _serde::__private::Ok(__Field::__field0),
                                    "length_more_than" => _serde::__private::Ok(__Field::__field1),
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
                                    b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                                    b"length_more_than" => _serde::__private::Ok(__Field::__field1),
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
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<
                                #postgresql_type_or_json_type_ident_where_element_length_more_than_upper_camel_case,
                            >,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #postgresql_type_or_json_type_ident_where_element_length_more_than_upper_camel_case;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter<'_>,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    #struct_postgresql_type_or_json_type_ident_where_element_length_more_than_double_quotes_token_stream,
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
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    crate::LogicalOperator,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &#struct_postgresql_type_or_json_type_ident_where_element_length_more_than_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    #std_primitive_i64_token_stream,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &#struct_postgresql_type_or_json_type_ident_where_element_length_more_than_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                match #postgresql_type_or_json_type_ident_where_element_length_more_than_upper_camel_case::try_new(__field0, __field1) {
                                    Ok(value) => _serde::__private::Ok(value),
                                    Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                }
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<
                                    crate::LogicalOperator,
                                > = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<#std_primitive_i64_token_stream> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "logical_operator",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    crate::LogicalOperator,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "length_more_than",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    #std_primitive_i64_token_stream,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("logical_operator")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("length_more_than")?
                                    }
                                };
                                match #postgresql_type_or_json_type_ident_where_element_length_more_than_upper_camel_case::try_new(__field0, __field1) {
                                    Ok(value) => _serde::__private::Ok(value),
                                    Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                }
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "logical_operator",
                            "length_more_than",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #postgresql_type_or_json_type_ident_where_element_length_more_than_double_quotes_token_stream,
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    #postgresql_type_or_json_type_ident_where_element_length_more_than_upper_camel_case,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
        }
    }
    fn generate_additional_type_declaration_token_stream() -> proc_macro2::TokenStream {
        let length_more_than_snake_case = naming::LengthMoreThanSnakeCase;
        let std_primitive_i64_token_stream = Self::std_primitive_i64_token_stream();
        quote::quote!{
            #length_more_than_snake_case: #std_primitive_i64_token_stream,
        }
    }
    fn generate_additional_default_initialization_token_stream() -> proc_macro2::TokenStream {
        let length_more_than_snake_case = naming::LengthMoreThanSnakeCase;
        let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
        quote::quote!{length_more_than: #core_default_default_default}
    }
    fn generate_try_generate_bind_increments_token_stream(postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType) -> proc_macro2::TokenStream {
        let increment_snake_case = naming::IncrementSnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let format_handle_token_stream = {
            let function = match &postgresql_type_or_json_type {
                crate::PostgresqlTypeOrJsonType::PostgresqlType => "length",
                crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => "jsonb_array_length",
            };
            generate_quotes::double_quotes_token_stream(&format!("{{}}({function}({{}}) > ${{}})"))
        };
        quote::quote!{
            match #increment_snake_case.checked_add(1) {
                Some(#value_snake_case) => {
                    *#increment_snake_case = #value_snake_case;
                    Ok(format!(#format_handle_token_stream, &self.logical_operator.to_query_part(is_need_to_add_logical_operator), #column_snake_case, #increment_snake_case))
                }
                None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
            }
        }
    }
    fn generate_bind_value_to_query_token_stream() -> proc_macro2::TokenStream {
        let length_more_than_snake_case = naming::LengthMoreThanSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        quote::quote!{
            #query_snake_case = #query_snake_case.bind(self.#length_more_than_snake_case);
            #query_snake_case
        }
    }
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
        let additional_type_declaration_token_stream = Self::generate_additional_type_declaration_token_stream();
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &self_upper_camel_case,
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &ident,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &Self::generate_try_new_error_named_variants_token_stream(),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &Self::generate_try_new_content_token_stream(
                    &ident,
                    &postgresql_type_or_json_type,
                ),
                impl_deserialize_token_stream: &self.generate_impl_deserialize_token_stream(
                    &ident,
                    &postgresql_type_or_json_type,
                )
            },
            &additional_type_declaration_token_stream,
            &Self::generate_additional_default_initialization_token_stream(),
            &Self::generate_try_generate_bind_increments_token_stream(&postgresql_type_or_json_type),
            &Self::generate_bind_value_to_query_token_stream()
        )
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        variant: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&variant));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
        let additional_type_declaration_token_stream = Self::generate_additional_type_declaration_token_stream();
        crate::generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &postgresql_type_or_json_type,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &variant,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &Self::generate_try_new_error_named_variants_token_stream(),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &Self::generate_try_new_content_token_stream(
                    &variant,
                    &postgresql_type_or_json_type,
                ),
                impl_deserialize_token_stream: &self.generate_impl_deserialize_token_stream(
                    &variant,
                    &postgresql_type_or_json_type,
                )
            },
            &crate::ShouldImplementSchemarsJsonSchema::True,
            &additional_type_declaration_token_stream,
            &Self::generate_additional_default_initialization_token_stream(),
            &Self::generate_try_generate_bind_increments_token_stream(&postgresql_type_or_json_type),
            &Self::generate_bind_value_to_query_token_stream()
        )
    }
}

pub struct EqualToEncodedStringRepresentation;
impl crate::WhereOperatorName for EqualToEncodedStringRepresentation {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::EqualToEncodedStringRepresentationUpperCamelCase
    }
}
impl EqualToEncodedStringRepresentation {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{
                pub encode_format: EncodeFormat,
                pub encoded_string_representation: std::string::String,
            },
            &{
                let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                quote::quote!{
                    encode_format: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    encoded_string_representation: #core_default_default_default,
                }
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!("{}(encode({}, '{}') = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), #column_snake_case, &self.encode_format, #increment_snake_case))
                    }
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.encoded_string_representation);
                #query_snake_case
            }
        )
    }
}

pub struct ValueIsContainedWithinRange;
impl crate::WhereOperatorName for ValueIsContainedWithinRange {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::ValueIsContainedWithinRangeUpperCamelCase
    }
}
impl ValueIsContainedWithinRange {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
        range_type_token_stream: &dyn quote::ToTokens,
        range_type_should_impl_range_length: &crate::ShouldImplRangeLength,
        range_type_default_initialization_token_stream: &dyn quote::ToTokens,
        range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #range_type_token_stream},
            &quote::quote!{#value_snake_case: #range_type_default_initialization_token_stream},
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} @> ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case #range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream);
                #query_snake_case
            }
        )
    }
}

pub struct ContainsAnotherRange;
impl crate::WhereOperatorName for ContainsAnotherRange {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::ContainsAnotherRangeUpperCamelCase
    }
}
impl ContainsAnotherRange {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} @> ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct StrictlyToLeftOfRange;
impl crate::WhereOperatorName for StrictlyToLeftOfRange {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::StrictlyToLeftOfRangeUpperCamelCase
    }
}
impl StrictlyToLeftOfRange {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} &< ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct StrictlyToRightOfRange;
impl crate::WhereOperatorName for StrictlyToRightOfRange {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::StrictlyToRightOfRangeUpperCamelCase
    }
}
impl StrictlyToRightOfRange {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} &> ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct IncludedLowerBound;
impl crate::WhereOperatorName for IncludedLowerBound {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::IncludedLowerBoundUpperCamelCase
    }
}
impl IncludedLowerBound {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
        range_type_token_stream: &dyn quote::ToTokens,
        range_type_default_initialization_token_stream: &dyn quote::ToTokens,
        range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        crate::generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #range_type_token_stream},
            &quote::quote!{#value_snake_case: #range_type_default_initialization_token_stream},
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}(lower({}) = ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case #range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream);
                #query_snake_case
            }
        )
    }
}