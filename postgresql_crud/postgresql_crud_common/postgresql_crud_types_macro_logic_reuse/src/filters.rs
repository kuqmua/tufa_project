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