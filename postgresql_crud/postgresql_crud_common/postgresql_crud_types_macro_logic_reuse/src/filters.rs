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