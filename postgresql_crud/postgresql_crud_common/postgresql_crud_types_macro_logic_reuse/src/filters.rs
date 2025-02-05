pub trait WhereOperatorName {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens;
}

enum IsNullablePostgresqlType<'a> {
    NullablePostgresqlType {
        where_operator_type: &'a crate::WhereOperatorType<'a>,
    },
    NotNullPostgresqlType {
        where_operator_type: &'a crate::WhereOperatorType<'a>,
    },
    PostgresqlJsonType,
}

fn generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
    ident: &dyn quote::ToTokens,
    postfix: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    is_nullable: &crate::IsNullable,
    should_where_element_fields_be_public: crate::ShouldWhereElementFieldsBePublic,
    additional_type_declaration_token_stream: &dyn quote::ToTokens,
    additional_default_initialization_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_try_generate_bind_increments_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_bind_value_to_query_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let generate_postgresql_type_ident_where_element_tokens_upper_camel_case = |prefix: &dyn std::fmt::Display|{
        let value = format!("{prefix}{postfix}");
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let postgresql_type_ident_where_element_tokens_upper_camel_case = generate_postgresql_type_ident_where_element_tokens_upper_camel_case(
        &naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident)
    );
    let postgresql_type_std_option_option_ident_where_element_tokens_upper_camel_case = generate_postgresql_type_ident_where_element_tokens_upper_camel_case(
        &naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementUpperCamelCase::from_tokens(&ident)
    );
    match &is_nullable {
        crate::IsNullable::True => macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(
            &postgresql_type_std_option_option_ident_where_element_tokens_upper_camel_case,
            &postgresql_type_ident_where_element_tokens_upper_camel_case
        ),
        crate::IsNullable::False => generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &crate::PostgresqlTypeOrJsonType::PostgresqlType,
            &postgresql_type_ident_where_element_tokens_upper_camel_case,
            should_where_element_fields_be_public,
            &crate::ShouldDeriveSchemarsJsonSchema::False,
            &additional_type_declaration_token_stream,
            &additional_default_initialization_token_stream,
            &postgresql_type_self_where_try_generate_bind_increments_token_stream,
            &postgresql_type_self_where_bind_value_to_query_token_stream,
        )
    }
}

fn generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    postgresql_type_or_postgresql_json_type_ident_where_element_tokens_upper_camel_case: &dyn quote::ToTokens,
    should_where_element_fields_be_public: crate::ShouldWhereElementFieldsBePublic,
    should_implement_schemars_json_schema: &crate::ShouldDeriveSchemarsJsonSchema,
    additional_type_declaration_token_stream: &dyn quote::ToTokens,
    additional_default_initialization_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_try_generate_bind_increments_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_bind_value_to_query_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_ident_where_element_tokens_token_stream = crate::generate_postgresql_type_tokens_where_element_tokens_token_stream(
        &postgresql_type_or_postgresql_json_type_ident_where_element_tokens_upper_camel_case,
        &should_where_element_fields_be_public,
        &should_implement_schemars_json_schema,
        &additional_type_declaration_token_stream,
    );
    let maybe_try_new_error_named_and_try_new_and_deserialize_token_stream = should_where_element_fields_be_public.maybe_generate_try_new_error_named_and_try_new_and_deserialize_token_stream(postgresql_type_or_json_type);
    let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_tokens_token_stream = crate::generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
        &postgresql_type_or_postgresql_json_type_ident_where_element_tokens_upper_camel_case,
        &{
            let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
                crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
            };
            quote::quote! {Self {
                logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                #additional_default_initialization_token_stream
            }}
        },
    );
    let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_tokens_token_stream = crate::generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_token_stream(
        &postgresql_type_or_postgresql_json_type_ident_where_element_tokens_upper_camel_case,
        &postgresql_type_self_where_try_generate_bind_increments_token_stream,
        &postgresql_type_self_where_bind_value_to_query_token_stream
    );
    quote::quote! {
        #postgresql_type_ident_where_element_tokens_token_stream
        #maybe_try_new_error_named_and_try_new_and_deserialize_token_stream
        #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_tokens_token_stream
        #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_tokens_token_stream
    }
}

pub struct Equal;
impl WhereOperatorName for Equal {
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
    fn generate_try_generate_bind_increments_token_stream(is_nullable_postgresql_type: &IsNullablePostgresqlType) -> proc_macro2::TokenStream {
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
            IsNullablePostgresqlType::NullablePostgresqlType {
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
            IsNullablePostgresqlType::NotNullPostgresqlType { where_operator_type: _, } => match_increment_checked_add_token_stream,
            IsNullablePostgresqlType::PostgresqlJsonType => match_increment_checked_add_token_stream,
        }
    }
    fn generate_bind_value_to_query_token_stream(is_nullable_postgresql_type: &IsNullablePostgresqlType) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let generate_query_equals_query_bind_token_stream = |bind_content_token_stream: &proc_macro2::TokenStream|{
            quote::quote!{
                #query_snake_case = #query_snake_case.bind(#bind_content_token_stream);
            }
        };
        let additional_content_token_stream = match &is_nullable_postgresql_type {
            IsNullablePostgresqlType::NullablePostgresqlType {
                where_operator_type,
            } => {
                let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
                quote::quote!{
                    if let Some(#value_snake_case) = self.#value_snake_case {
                        #query_snake_case = #query_snake_case.bind(#value_snake_case #where_operator_type_additional_bind_token_stream);
                    }
                }
            },
            IsNullablePostgresqlType::NotNullPostgresqlType {
                where_operator_type,
            } => generate_query_equals_query_bind_token_stream(&{
                let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
                quote::quote!{self.#value_snake_case #where_operator_type_additional_bind_token_stream}
            }),
            //todo maybe instead of wrapping into sqlx::types::Json - impl Encode? 
            IsNullablePostgresqlType::PostgresqlJsonType => generate_query_equals_query_bind_token_stream(&quote::quote!{sqlx::types::Json(self.#value_snake_case)}),
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
            let postfix: &dyn naming::StdFmtDisplayPlusQuoteToTokens = WhereOperatorName::upper_camel_case(self);
            let value = format!("{prefix}{postfix}");
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
        let should_where_element_fields_be_public = crate::ShouldWhereElementFieldsBePublic::True;
        let should_implement_schemars_json_schema = crate::ShouldDeriveSchemarsJsonSchema::False;
        match &is_nullable {
            crate::IsNullable::True => {
                let is_nullable_postgresql_type = IsNullablePostgresqlType::NullablePostgresqlType {
                    where_operator_type: &where_operator_type,
                };
                generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
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
                let is_nullable_postgresql_type = IsNullablePostgresqlType::NotNullPostgresqlType {
                    where_operator_type: &where_operator_type,
                };
                generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
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
        postgresql_json_type: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_json_type));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let is_nullable_postgresql_type = IsNullablePostgresqlType::PostgresqlJsonType;
        let (
            postgresql_json_type_handle,
            postgresql_json_type_pattern
        ) = postgresql_json_type.to_postgresql_json_type_handle_and_postgresql_json_type_pattern();
        generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &crate::PostgresqlTypeOrJsonType::PostgresqlJsonType,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::True,
            &crate::ShouldDeriveSchemarsJsonSchema::True,
            &Self::generate_additional_type_declaration_token_stream(&postgresql_json_type),
            &Self::generate_additional_default_initialization_token_stream(&{
                let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                    let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
                    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
                    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
                    quote::quote! {
                        crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
                    }
                };
                crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream
            }),
            &Self::generate_try_generate_bind_increments_token_stream(&is_nullable_postgresql_type),
            &Self::generate_bind_value_to_query_token_stream(&is_nullable_postgresql_type),
        )
    }
}

pub struct GreaterThan;
impl WhereOperatorName for GreaterThan {
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
        generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}({} > ${})"})
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
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
        postgresql_json_type: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_json_type));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let (
            postgresql_json_type_handle,
            postgresql_json_type_pattern
        ) = postgresql_json_type.to_postgresql_json_type_handle_and_postgresql_json_type_pattern();
        generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &crate::PostgresqlTypeOrJsonType::PostgresqlJsonType,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::True,
            &crate::ShouldDeriveSchemarsJsonSchema::True,
            &Self::generate_additional_type_declaration_token_stream(&postgresql_json_type_pattern.field_type(&postgresql_json_type_handle)),
            &Self::generate_additional_default_initialization_token_stream(&postgresql_json_type_pattern.initialization_token_stream(&postgresql_json_type_handle)),
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
impl WhereOperatorName for Between {
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
        ) = crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_element_between_upper_camel_case, 3);
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
        let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
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
        postgresql_json_type: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_json_type));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
        let (
            postgresql_json_type_handle,
            postgresql_json_type_pattern
        ) = postgresql_json_type.to_postgresql_json_type_handle_and_postgresql_json_type_pattern();
        let non_optional_field_type = postgresql_json_type_pattern.non_optional_field_type(&postgresql_json_type_handle);
        let additional_type_declaration_token_stream = Self::generate_additional_type_declaration_token_stream(&non_optional_field_type);
        generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &postgresql_type_or_json_type,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &postgresql_json_type,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &Self::generate_try_new_error_named_variants_token_stream(
                    &between_try_new_error_type,
                    &non_optional_field_type,
                ),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &Self::generate_try_new_content_token_stream(
                    &postgresql_json_type,
                    &postgresql_type_or_json_type,
                    &between_try_new_error_type,
                    &proc_macro2::TokenStream::new(),
                ),
                impl_deserialize_token_stream: &Self::generate_impl_deserialize_token_stream(
                    &postgresql_json_type,
                    &postgresql_type_or_json_type,
                    &non_optional_field_type,
                ),
            },
            &crate::ShouldDeriveSchemarsJsonSchema::True,
            &additional_type_declaration_token_stream,
            &Self::generate_additional_default_initialization_token_stream(&crate::PostgresqlJsonTypePattern::from(postgresql_json_type).non_optional_initialization_token_stream(&postgresql_json_type_handle)),
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

fn generate_try_new_error_named_variants_token_stream_c7fb6522_d391_4af8_b39f_9c6611d199c5(not_unique_value_type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
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
fn generate_try_new_content_token_stream_83fd9d8a_51ab_4c3c_97fc_be2258f730fc(
    ident: &dyn quote::ToTokens,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    filter: &dyn std::fmt::Display,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let is_empty_upper_camel_case = naming::IsEmptyUpperCamelCase;
    let not_unique_upper_camel_case = naming::NotUniqueUpperCamelCase;
    let element_snake_case = naming::ElementSnakeCase;
    let acc_snake_case = naming::AccSnakeCase;
    let postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_try_new_error_named_upper_camel_case = {
        let value = format!(
            "{postgresql_type_or_json_type}{}{}{}",
            &naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident),
            filter,
            naming::TryNewErrorNamedUpperCamelCase
        );
        value.parse::<proc_macro2::TokenStream>().unwrap()
    };
    quote::quote!{
        if #value_snake_case.is_empty() {
            return Err(#postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_try_new_error_named_upper_camel_case::#is_empty_upper_camel_case {
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        {
            let mut #acc_snake_case = vec![];
            for #element_snake_case in &#value_snake_case {
                if !#acc_snake_case.contains(&#element_snake_case) {
                    #acc_snake_case.push(#element_snake_case);
                } else {
                    return Err(#postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_try_new_error_named_upper_camel_case::#not_unique_upper_camel_case {
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
fn generate_impl_deserialize_token_stream_45e71974_8062_4f69_9296_248692666dde(
    ident: &dyn quote::ToTokens,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    vec_type_token_stream: &dyn quote::ToTokens,
    filter: &dyn std::fmt::Display,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_upper_camel_case: &dyn quote::ToTokens = &{
        let value = format!(
            "{postgresql_type_or_json_type}{}{}",
            naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident),
            filter
        );
        value.parse::<proc_macro2::TokenStream>().unwrap()
    };
    let (
        struct_postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_double_quotes_token_stream,
        struct_postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_with_2_elements_double_quotes_token_stream,
        postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_double_quotes_token_stream
    ) = crate::generate_serde_deserialize_double_quotes_token_stream(&format!("{}", quote::quote!{#postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_upper_camel_case}), 2);
    quote::quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_upper_camel_case {
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
                            #postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_upper_camel_case,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_upper_camel_case;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter<'_>,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                #struct_postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_double_quotes_token_stream,
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
                                            &#struct_postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_with_2_elements_double_quotes_token_stream,
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
                                            &#struct_postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_with_2_elements_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            match #postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_upper_camel_case::try_new(__field0, __field1) {
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
                            match #postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_upper_camel_case::try_new(__field0, __field1) {
                                Ok(value) => _serde::__private::Ok(value),
                                Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                            }
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        #postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_double_quotes_token_stream,
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                #postgresql_type_or_json_type_ident_where_element_contains_all_elements_of_array_upper_camel_case,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
}
fn generate_additional_type_declaration_token_stream_0bec460b_9b9e_46ba_9b8f_42392c528735(vec_type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote!{
        #value_snake_case: std::vec::Vec<#vec_type_token_stream>
    }
}
fn generate_additional_default_initialization_token_stream_e46171d1_2c28_450b_85a8_80293ebe69f8(default_initialization_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote!{
        #value_snake_case: vec![#default_initialization_token_stream]
    }
}

pub struct In;
impl WhereOperatorName for In {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::InUpperCamelCase
    }
}
impl In {
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
        let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
        let where_operator_type_type_token_stream = where_operator_type.type_token_stream();
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
        let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_0bec460b_9b9e_46ba_9b8f_42392c528735(&where_operator_type_type_token_stream);
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &self_upper_camel_case,
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &ident,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_c7fb6522_d391_4af8_b39f_9c6611d199c5(&where_operator_type_type_token_stream),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &generate_try_new_content_token_stream_83fd9d8a_51ab_4c3c_97fc_be2258f730fc(
                    &ident,
                    &postgresql_type_or_json_type,
                    &self_upper_camel_case,
                ),
                impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_45e71974_8062_4f69_9296_248692666dde(
                    &ident,
                    &postgresql_type_or_json_type,
                    &where_operator_type_type_token_stream,
                    &self_upper_camel_case,
                ),
            },
            &additional_type_declaration_token_stream,
            &generate_additional_default_initialization_token_stream_e46171d1_2c28_450b_85a8_80293ebe69f8(&where_operator_type.default_initialization_token_stream()),
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
        postgresql_json_type: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        let (
            postgresql_json_type_handle,
            postgresql_json_type_pattern
        ) = postgresql_json_type.to_postgresql_json_type_handle_and_postgresql_json_type_pattern();
        let field_type = postgresql_json_type_pattern.field_type(&postgresql_json_type_handle);

        let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_json_type));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let non_optional_field_type = postgresql_json_type_pattern.non_optional_field_type(&postgresql_json_type_handle);
        let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_0bec460b_9b9e_46ba_9b8f_42392c528735(&non_optional_field_type);
        generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &postgresql_type_or_json_type,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &postgresql_json_type,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_c7fb6522_d391_4af8_b39f_9c6611d199c5(&non_optional_field_type),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &generate_try_new_content_token_stream_83fd9d8a_51ab_4c3c_97fc_be2258f730fc(
                    &postgresql_json_type,
                    &postgresql_type_or_json_type,
                    &self_upper_camel_case,
                ),
                impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_45e71974_8062_4f69_9296_248692666dde(
                    &postgresql_json_type,
                    &postgresql_type_or_json_type,
                    &non_optional_field_type,
                    &self_upper_camel_case,
                ),
            },
            &crate::ShouldDeriveSchemarsJsonSchema::True,
            &additional_type_declaration_token_stream,
            &generate_additional_default_initialization_token_stream_e46171d1_2c28_450b_85a8_80293ebe69f8(&postgresql_json_type_pattern.non_optional_initialization_token_stream(&postgresql_json_type_handle)),
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

fn generate_try_new_error_named_variants_token_stream_f2c321ab_f37c_41cb_b949_a7bef3f819b9() -> proc_macro2::TokenStream {
    let is_empty_upper_camel_case = naming::IsEmptyUpperCamelCase;
    quote::quote!{
        #is_empty_upper_camel_case {
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        }
    }
}
fn generate_try_new_content_token_stream_87985e52_7a0a_4c62_8328_cc23a015d7fd(
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
fn generate_impl_deserialize_token_stream_31de3e8b_5624_4906_b34f_e3e20968640c(
    ident: &dyn quote::ToTokens,
    regular_expression: &RegularExpression,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
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
    ) = crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_element_regular_expression_upper_camel_case, 2);
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
fn generate_additional_type_declaration_token_stream_10e900fc_d0fd_45ae_9d09_d4042ebf37c9() -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let std_string_string = token_patterns::StdStringString;
    quote::quote!{#value_snake_case: #std_string_string}
}
fn generate_additional_default_initialization_token_stream_ca12ef3a_dd45_4195_9f05_03241c037d04() -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
    quote::quote!{#value_snake_case: #core_default_default_default}
}
fn generate_postgresql_type_self_where_try_generate_bind_increments_token_stream_2cc03745_9c77_4be3_8819_e948868a7603(
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
fn generate_postgresql_type_self_where_bind_value_to_query_token_stream_9e47be54_cd7f_49bf_a8fa_39dae7512b3e() -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    quote::quote!{
        #query_snake_case = #query_snake_case.bind(self.#value_snake_case);
        #query_snake_case
    }
}
fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream_daf5b604_4dcd_4c10_b1cb_aebeb4f4011f(
    ident: &dyn quote::ToTokens,
    is_nullable: &crate::IsNullable,
    regular_expression: &RegularExpression,
    self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
    generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
        &ident,
        &self_upper_camel_case,
        &is_nullable,
        crate::ShouldWhereElementFieldsBePublic::False {
            ident: &ident,
            postfix: &self_upper_camel_case,
            try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_f2c321ab_f37c_41cb_b949_a7bef3f819b9(),
            try_new_additional_input_parameters_token_stream: &generate_additional_type_declaration_token_stream_10e900fc_d0fd_45ae_9d09_d4042ebf37c9(),
            try_new_content_token_stream: &generate_try_new_content_token_stream_87985e52_7a0a_4c62_8328_cc23a015d7fd(
                &ident,
                &regular_expression,
                &postgresql_type_or_json_type,
            ),
            impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_31de3e8b_5624_4906_b34f_e3e20968640c(
                &ident,
                &regular_expression,
                &postgresql_type_or_json_type,
            ),
        },
        &generate_additional_type_declaration_token_stream_10e900fc_d0fd_45ae_9d09_d4042ebf37c9(),
        &generate_additional_default_initialization_token_stream_ca12ef3a_dd45_4195_9f05_03241c037d04(),
        &generate_postgresql_type_self_where_try_generate_bind_increments_token_stream_2cc03745_9c77_4be3_8819_e948868a7603(&regular_expression, &postgresql_type_or_json_type),
        &generate_postgresql_type_self_where_bind_value_to_query_token_stream_9e47be54_cd7f_49bf_a8fa_39dae7512b3e()
    )
}
fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_8b498c22_facc_478f_9fef_2d155c68b6fa(
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
    generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
        &crate::PostgresqlTypeOrJsonType::PostgresqlJsonType,
        &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
        crate::ShouldWhereElementFieldsBePublic::False {
            ident: &ident,
            postfix: &self_upper_camel_case,
            try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_f2c321ab_f37c_41cb_b949_a7bef3f819b9(),
            try_new_additional_input_parameters_token_stream: &generate_additional_type_declaration_token_stream_10e900fc_d0fd_45ae_9d09_d4042ebf37c9(),
            try_new_content_token_stream: &generate_try_new_content_token_stream_87985e52_7a0a_4c62_8328_cc23a015d7fd(
                &ident,
                &regular_expression,
                &postgresql_type_or_json_type,
            ),
            impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_31de3e8b_5624_4906_b34f_e3e20968640c(
                &ident,
                &regular_expression,
                &postgresql_type_or_json_type,
            ),
        },
        &crate::ShouldDeriveSchemarsJsonSchema::True,
        &generate_additional_type_declaration_token_stream_10e900fc_d0fd_45ae_9d09_d4042ebf37c9(),
        &generate_additional_default_initialization_token_stream_ca12ef3a_dd45_4195_9f05_03241c037d04(),
        &generate_postgresql_type_self_where_try_generate_bind_increments_token_stream_2cc03745_9c77_4be3_8819_e948868a7603(&regular_expression, &postgresql_type_or_json_type),
        &generate_postgresql_type_self_where_bind_value_to_query_token_stream_9e47be54_cd7f_49bf_a8fa_39dae7512b3e()
    )
}
pub struct CaseSensitiveRegularExpression;
impl WhereOperatorName for CaseSensitiveRegularExpression {
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
        generate_postgresql_type_tokens_where_element_variant_handle_token_stream_daf5b604_4dcd_4c10_b1cb_aebeb4f4011f(
            &ident,
            &is_nullable,
            &RegularExpression::CaseSensitive,
            WhereOperatorName::upper_camel_case(self),
        )
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_8b498c22_facc_478f_9fef_2d155c68b6fa(
            &ident,
            &RegularExpression::CaseSensitive,
            WhereOperatorName::upper_camel_case(self),
        )
    }
}
pub struct CaseInsensitiveRegularExpression;
impl WhereOperatorName for CaseInsensitiveRegularExpression {
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
        generate_postgresql_type_tokens_where_element_variant_handle_token_stream_daf5b604_4dcd_4c10_b1cb_aebeb4f4011f(
            &ident,
            &is_nullable,
            &RegularExpression::CaseInsensitive,
            WhereOperatorName::upper_camel_case(self),
        )
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_8b498c22_facc_478f_9fef_2d155c68b6fa(
            &ident,
            &RegularExpression::CaseInsensitive,
            WhereOperatorName::upper_camel_case(self),
        )
    }
}

pub struct Before;
impl WhereOperatorName for Before {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}({} < ${})"}),
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct CurrentDate;
impl WhereOperatorName for CurrentDate {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
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
impl WhereOperatorName for GreaterThanCurrentDate {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
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
impl WhereOperatorName for CurrentTimestamp {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
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
impl WhereOperatorName for GreaterThanCurrentTimestamp {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
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
impl WhereOperatorName for CurrentTime {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
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
impl WhereOperatorName for GreaterThanCurrentTime {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
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

fn generate_try_new_error_named_variants_token_stream_e2eea6c9_aaf9_486b_852b_d0ae24c7f519() -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let length_is_negative_upper_camel_case = naming::LengthIsNegativeUpperCamelCase;
    let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;//todo maybe use i32 for now? just to remove possible js number max value overflow
    quote::quote!{
        #length_is_negative_upper_camel_case {
            #[eo_to_std_string_string_serialize_deserialize]
            #value_snake_case: #std_primitive_i64_token_stream,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
}
fn generate_try_new_content_token_stream_dcd7743b_821a_494a_8657_bbd7a8831020(
    ident: &dyn quote::ToTokens,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let postgresql_type_or_json_type_ident_where_element_length_more_than_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match &postgresql_type_or_json_type {
        crate::PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementLengthMoreThanTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementLengthMoreThanTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
    };
    let length_more_than_snake_case = naming::LengthMoreThanSnakeCase;
    let length_is_negative_upper_camel_case = naming::LengthIsNegativeUpperCamelCase;
    quote::quote!{
        if #length_more_than_snake_case >= 0 {
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
fn generate_impl_deserialize_token_stream_d64c9ce5_9574_4d05_9cae_b17a39b681e6(
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
    ) = crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_element_length_more_than_upper_camel_case, 2);
    let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
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
fn generate_additional_type_declaration_token_stream_3dae697f_0b51_4a93_901f_6cef909d74e7() -> proc_macro2::TokenStream {
    let length_more_than_snake_case = naming::LengthMoreThanSnakeCase;
    let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
    quote::quote!{
        #length_more_than_snake_case: #std_primitive_i64_token_stream,
    }
}
fn generate_additional_default_initialization_token_stream_432a20e5_249e_4ea9_b0b3_4a9b4937dd9f() -> proc_macro2::TokenStream {
    let length_more_than_snake_case = naming::LengthMoreThanSnakeCase;
    let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
    quote::quote!{length_more_than: #core_default_default_default}
}
enum LengthOperator {
    Eq,
    More,
}
impl LengthOperator {
    fn postgresql_operator(&self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::More => ">",
        }
    }
}
fn generate_try_generate_bind_increments_token_stream_145071b7_6a73_4d42_938b_cd3e0133f4bf(postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType, length_operator: &LengthOperator) -> proc_macro2::TokenStream {
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
        generate_quotes::double_quotes_token_stream(&format!("{{}}({function}({{}}) {} ${{}})", length_operator.postgresql_operator()))
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
fn generate_bind_value_to_query_token_stream_724cfe40_ee21_4d5e_adbe_954dc62a5139() -> proc_macro2::TokenStream {
    let length_more_than_snake_case = naming::LengthMoreThanSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    quote::quote!{
        #query_snake_case = #query_snake_case.bind(self.#length_more_than_snake_case);
        #query_snake_case
    }
}
fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream_81c7837d_3661_4676_a793_fb84ceaf778e(
    self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    ident: &dyn quote::ToTokens,
    is_nullable: &crate::IsNullable,
    length_operator: &LengthOperator,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
    let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_3dae697f_0b51_4a93_901f_6cef909d74e7();
    generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
        &ident,
        &self_upper_camel_case,
        &is_nullable,
        crate::ShouldWhereElementFieldsBePublic::False {
            ident: &ident,
            postfix: &self_upper_camel_case,
            try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_e2eea6c9_aaf9_486b_852b_d0ae24c7f519(),
            try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
            try_new_content_token_stream: &generate_try_new_content_token_stream_dcd7743b_821a_494a_8657_bbd7a8831020(
                &ident,
                &postgresql_type_or_json_type,
            ),
            impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_d64c9ce5_9574_4d05_9cae_b17a39b681e6(
                &ident,
                &postgresql_type_or_json_type,
            )
        },
        &additional_type_declaration_token_stream,
        &generate_additional_default_initialization_token_stream_432a20e5_249e_4ea9_b0b3_4a9b4937dd9f(),
        &generate_try_generate_bind_increments_token_stream_145071b7_6a73_4d42_938b_cd3e0133f4bf(&postgresql_type_or_json_type, length_operator),
        &generate_bind_value_to_query_token_stream_724cfe40_ee21_4d5e_adbe_954dc62a5139()
    )
}
fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_61c9bbf6_474d_4040_b849_f12e84e8e47e(
    self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    postgresql_json_type: &crate::PostgresqlJsonType,
    length_operator: &LengthOperator,
) -> proc_macro2::TokenStream {
    let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
        let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_json_type));
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
    let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_3dae697f_0b51_4a93_901f_6cef909d74e7();
    generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
        &postgresql_type_or_json_type,
        &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
        crate::ShouldWhereElementFieldsBePublic::False {
            ident: &postgresql_json_type,
            postfix: &self_upper_camel_case,
            try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_e2eea6c9_aaf9_486b_852b_d0ae24c7f519(),
            try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
            try_new_content_token_stream: &generate_try_new_content_token_stream_dcd7743b_821a_494a_8657_bbd7a8831020(
                &postgresql_json_type,
                &postgresql_type_or_json_type,
            ),
            impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_d64c9ce5_9574_4d05_9cae_b17a39b681e6(
                &postgresql_json_type,
                &postgresql_type_or_json_type,
            )
        },
        &crate::ShouldDeriveSchemarsJsonSchema::True,
        &additional_type_declaration_token_stream,
        &generate_additional_default_initialization_token_stream_432a20e5_249e_4ea9_b0b3_4a9b4937dd9f(),
        &generate_try_generate_bind_increments_token_stream_145071b7_6a73_4d42_938b_cd3e0133f4bf(&postgresql_type_or_json_type, length_operator),
        &generate_bind_value_to_query_token_stream_724cfe40_ee21_4d5e_adbe_954dc62a5139()
    )
}

pub struct LengthMoreThan;
impl WhereOperatorName for LengthMoreThan {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::LengthMoreThanUpperCamelCase
    }
}
impl LengthMoreThan {
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        generate_postgresql_type_tokens_where_element_variant_handle_token_stream_81c7837d_3661_4676_a793_fb84ceaf778e(
            &WhereOperatorName::upper_camel_case(self),
            ident,
            is_nullable,
            &LengthOperator::More,
        )
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        postgresql_json_type: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_61c9bbf6_474d_4040_b849_f12e84e8e47e(
            &WhereOperatorName::upper_camel_case(self),
            postgresql_json_type,
            &LengthOperator::More,
        )
    }
}

pub struct EqualToEncodedStringRepresentation;
impl WhereOperatorName for EqualToEncodedStringRepresentation {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &WhereOperatorName::upper_camel_case(self),
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
                        Ok(format!(
                            "{}(encode({}, '{}') = ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            &self.encode_format,
                            #increment_snake_case
                        ))
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
impl WhereOperatorName for ValueIsContainedWithinRange {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #range_type_token_stream},
            &quote::quote!{#value_snake_case: #range_type_default_initialization_token_stream},
            &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}({} @> ${})"}),
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case #range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream);
                #query_snake_case
            }
        )
    }
}

pub struct ContainsAnotherRange;
impl WhereOperatorName for ContainsAnotherRange {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}({} @> ${})"}),
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct StrictlyToLeftOfRange;
impl WhereOperatorName for StrictlyToLeftOfRange {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}({} &< ${})"}),
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct StrictlyToRightOfRange;
impl WhereOperatorName for StrictlyToRightOfRange {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}({} &> ${})"}),
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct IncludedLowerBound;
impl WhereOperatorName for IncludedLowerBound {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #range_type_token_stream},
            &quote::quote!{#value_snake_case: #range_type_default_initialization_token_stream},
            &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}(lower({}) = ${})"}),
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case #range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream);
                #query_snake_case
            }
        )
    }
}

pub struct ExcludedUpperBound;
impl WhereOperatorName for ExcludedUpperBound {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::ExcludedUpperBoundUpperCamelCase
    }
}
impl ExcludedUpperBound {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #range_type_token_stream},
            &quote::quote!{#value_snake_case: #range_type_default_initialization_token_stream},
            &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}(upper({}) = ${})"}),
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case #range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream);
                #query_snake_case
            }
        )
    }
}

pub struct GreaterThanLowerBound;
impl WhereOperatorName for GreaterThanLowerBound {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::GreaterThanLowerBoundUpperCamelCase
    }
}
impl GreaterThanLowerBound {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}({} > ${})"}),
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct OverlapWithRange;
impl WhereOperatorName for OverlapWithRange {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::OverlapWithRangeUpperCamelCase
    }
}
impl OverlapWithRange {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}({} && ${})"}),
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct AdjacentWithRange;
impl WhereOperatorName for AdjacentWithRange {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::AdjacentWithRangeUpperCamelCase
    }
}
impl AdjacentWithRange {
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
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            WhereOperatorName::upper_camel_case(self),
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&quote::quote!{"{}({} -|- ${})"}),
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}

pub struct RangeLength;
impl WhereOperatorName for RangeLength {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::RangeLengthUpperCamelCase
    }
}
impl RangeLength {
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
        let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
        let length_is_negative_or_zero_upper_camel_case = naming::LengthIsNegativeOrZeroUpperCamelCase;
        let std_primitive_i64_token_stream = quote::quote!{std::primitive::i64};
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &self_upper_camel_case,
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &ident,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &quote::quote!{
                    #length_is_negative_or_zero_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #value_snake_case: #std_primitive_i64_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                },
                try_new_additional_input_parameters_token_stream: &quote::quote!{
                    #value_snake_case: #std_primitive_i64_token_stream
                },
                try_new_content_token_stream: &{
                    let postgresql_type_ident_where_element_range_length_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementRangeLengthTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
                    quote::quote!{
                        if #value_snake_case > 0 {
                            Ok(Self {
                                logical_operator,
                                #value_snake_case,
                            })
                        }
                        else {
                            Err(#postgresql_type_ident_where_element_range_length_try_new_error_named_upper_camel_case::#length_is_negative_or_zero_upper_camel_case {
                                #value_snake_case,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            })
                        }
                    }
                },
                impl_deserialize_token_stream: &{
                    let postgresql_type_ident_where_element_range_length_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementRangeLengthUpperCamelCase::from_tokens(&ident);
                    let (
                        struct_postgresql_type_ident_where_element_range_length_double_quotes_token_stream,
                        struct_postgresql_type_ident_where_element_range_length_with_2_elements_double_quotes_token_stream,
                        postgresql_type_ident_where_element_range_length_double_quotes_token_stream
                    ) = crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_ident_where_element_range_length_upper_camel_case, 2);
                    quote::quote! {
                        const _: () = {
                            #[allow(unused_extern_crates, clippy::useless_attribute)]
                            extern crate serde as _serde;
                            #[automatically_derived]
                            impl<'de> _serde::Deserialize<'de> for #postgresql_type_ident_where_element_range_length_upper_camel_case {
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
                                            #postgresql_type_ident_where_element_range_length_upper_camel_case,
                                        >,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = #postgresql_type_ident_where_element_range_length_upper_camel_case;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private::Formatter,
                                        ) -> _serde::__private::fmt::Result {
                                            _serde::__private::Formatter::write_str(
                                                __formatter,
                                                #struct_postgresql_type_ident_where_element_range_length_double_quotes_token_stream,
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
                                                            &#struct_postgresql_type_ident_where_element_range_length_with_2_elements_double_quotes_token_stream,
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
                                                            &#struct_postgresql_type_ident_where_element_range_length_with_2_elements_double_quotes_token_stream,
                                                        ),
                                                    );
                                                }
                                            };
                                            match #postgresql_type_ident_where_element_range_length_upper_camel_case::try_new(__field0, __field1) {
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
                                                                <__A::Error as _serde::de::Error>::duplicate_field("value"),
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
                                                    _serde::__private::de::missing_field("value")?
                                                }
                                            };
                                            match #postgresql_type_ident_where_element_range_length_upper_camel_case::try_new(__field0, __field1) {
                                                Ok(value) => _serde::__private::Ok(value),
                                                Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                            }
                                        }
                                    }
                                    #[doc(hidden)]
                                    const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
                                    _serde::Deserializer::deserialize_struct(
                                        __deserializer,
                                        #postgresql_type_ident_where_element_range_length_double_quotes_token_stream,
                                        FIELDS,
                                        __Visitor {
                                            marker: _serde::__private::PhantomData::<
                                                #postgresql_type_ident_where_element_range_length_upper_camel_case,
                                            >,
                                            lifetime: _serde::__private::PhantomData,
                                        },
                                    )
                                }
                            }
                        };
                    }
                },
            },
            &quote::quote!{#value_snake_case: #std_primitive_i64_token_stream},//todo try_new - check length > 0
            &{
                let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                quote::quote!{#value_snake_case: #core_default_default_default}
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}(upper({}) - lower({}) = ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
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
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case);
                #query_snake_case
            }
        )
    }
}

fn generate_try_new_error_named_variants_token_stream_06af1515_1384_4d10_a4cf_aaf07284fd08() -> proc_macro2::TokenStream {
    let position_snake_case = naming::PositionSnakeCase;
    let position_is_less_than_zero_upper_camel_case = naming::PositionIsLessThanZeroUpperCamelCase;
    let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
    quote::quote!{
        #position_is_less_than_zero_upper_camel_case {
            #[eo_to_std_string_string_serialize_deserialize]
            #position_snake_case: #std_primitive_i32_token_stream,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
}
fn generate_try_new_content_token_stream_32f214a7_79ed_4fae_8fdd_0123bd84c652(
    ident: &dyn quote::ToTokens,
    postgresql_type_or_json_type_ident_where_element_position_filter_try_new_error_named_upper_camel_case: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let position_snake_case = naming::PositionSnakeCase;
    let position_is_less_than_zero_upper_camel_case = naming::PositionIsLessThanZeroUpperCamelCase;
    quote::quote!{
        if #position_snake_case >= 0 {
            Ok(Self {
                logical_operator,
                #value_snake_case,
                #position_snake_case,
            })
        }
        else {
            Err(#postgresql_type_or_json_type_ident_where_element_position_filter_try_new_error_named_upper_camel_case::#position_is_less_than_zero_upper_camel_case {
                #position_snake_case,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
fn generate_impl_deserialize_token_stream_f8893cac_9ce5_4333_81c7_6bfa8d56c65d(
    postgresql_type_or_json_type_ident_where_element_filter_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    value_type_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let (
        struct_postgresql_type_or_json_type_ident_where_element_filter_double_quotes_token_stream,
        struct_postgresql_type_or_json_type_ident_where_element_filter_with_2_elements_double_quotes_token_stream,
        postgresql_type_or_json_type_ident_where_element_filter_double_quotes_token_stream
    ) = crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_element_filter_upper_camel_case, 2);
    let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
    quote::quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for #postgresql_type_or_json_type_ident_where_element_filter_upper_camel_case {
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
                                "value" => _serde::__private::Ok(__Field::__field1),
                                "position" => _serde::__private::Ok(__Field::__field2),
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
                                b"position" => _serde::__private::Ok(__Field::__field2),
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
                            #postgresql_type_or_json_type_ident_where_element_filter_upper_camel_case,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type_or_json_type_ident_where_element_filter_upper_camel_case;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                #struct_postgresql_type_or_json_type_ident_where_element_filter_double_quotes_token_stream,
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
                                            &#struct_postgresql_type_or_json_type_ident_where_element_filter_with_2_elements_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                #value_type_token_stream,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &#struct_postgresql_type_or_json_type_ident_where_element_filter_with_2_elements_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                #std_primitive_i32_token_stream,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &#struct_postgresql_type_or_json_type_ident_where_element_filter_with_2_elements_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            match #postgresql_type_or_json_type_ident_where_element_filter_upper_camel_case::try_new(__field0, __field1, __field2) {
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
                            let mut __field1: _serde::__private::Option<#value_type_token_stream> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<#std_primitive_i32_token_stream> = _serde::__private::None;
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
                                                #value_type_token_stream,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "position",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                #std_primitive_i32_token_stream,
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
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("position")?
                                }
                            };
                            match #postgresql_type_or_json_type_ident_where_element_filter_upper_camel_case::try_new(__field0, __field1, __field2) {
                                Ok(value) => _serde::__private::Ok(value),
                                Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                            }
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "logical_operator",
                        "value",
                        "position",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        #postgresql_type_or_json_type_ident_where_element_filter_double_quotes_token_stream,
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                #postgresql_type_or_json_type_ident_where_element_filter_upper_camel_case,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
}
fn generate_additional_type_declaration_token_stream_34095bbb_d306_4a44_92e9_4df1a7354bc1(value_type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let position_snake_case = naming::PositionSnakeCase;
    let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
    quote::quote!{
        #value_snake_case: #value_type_token_stream,
        #position_snake_case: #std_primitive_i32_token_stream,
    }
}
fn generate_additional_default_initialization_token_stream_cd86231b_5ff2_4e91_9906_29b822838309(value_type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let position_snake_case = naming::PositionSnakeCase;
    let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
    quote::quote!{
        #value_snake_case: #value_type_token_stream,
        #position_snake_case: #core_default_default_default,
    }
}

pub struct BitVecPositionEquals;
impl WhereOperatorName for BitVecPositionEquals {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::BitVecPositionEqualsUpperCamelCase
    }
}
impl BitVecPositionEquals {
    fn generate_impl_deserialize_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    ) -> proc_macro2::TokenStream {
        let postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_or_json_type {
            crate::PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementBitVecPositionEqualsUpperCamelCase::from_tokens(&ident),
            crate::PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementBitVecPositionEqualsUpperCamelCase::from_tokens(&ident),
        };
        generate_impl_deserialize_token_stream_f8893cac_9ce5_4333_81c7_6bfa8d56c65d(
            &postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_upper_camel_case,
            &token_patterns::StdPrimitiveBool,
        )
    }
    fn generate_try_generate_bind_increments_token_stream() -> proc_macro2::TokenStream {
        let increment_snake_case = naming::IncrementSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        quote::quote!{
            match #increment_snake_case.checked_add(1) {
                Some(first_increment) => {
                    *#increment_snake_case = first_increment;
                    match #increment_snake_case.checked_add(1) {
                        Some(second_increment) => {
                            *#increment_snake_case = second_increment;
                            Ok(format!(
                                "{}(get_bit({}, ${}) = ${})",
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                #column_snake_case,
                                first_increment,
                                second_increment,
                            ))
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
    fn generate_bind_value_to_query_token_stream() -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let position_snake_case = naming::PositionSnakeCase;
        quote::quote!{
            #query_snake_case = #query_snake_case.bind(self.#position_snake_case);
            #query_snake_case = #query_snake_case.bind(if self.#value_snake_case {
                1
            }
            else {
                0
            });
            #query_snake_case
        }
    }
    pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &crate::IsNullable,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlType;
        let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_34095bbb_d306_4a44_92e9_4df1a7354bc1(&token_patterns::StdPrimitiveBool);
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &self_upper_camel_case,
            &is_nullable,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &ident,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_06af1515_1384_4d10_a4cf_aaf07284fd08(),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &generate_try_new_content_token_stream_9a677220_67b3_4d4d_a7b7_92314cce8e40(
                    &ident,
                    &postgresql_type_or_json_type,
                    &self_upper_camel_case,
                ),
                impl_deserialize_token_stream: &self.generate_impl_deserialize_token_stream(
                    &ident,
                    &postgresql_type_or_json_type,
                ),
            },
            &additional_type_declaration_token_stream,
            &generate_additional_default_initialization_token_stream_cd86231b_5ff2_4e91_9906_29b822838309(&token_patterns::CoreDefaultDefaultDefault),
            &Self::generate_try_generate_bind_increments_token_stream(),
            &Self::generate_bind_value_to_query_token_stream(),
        )
    }
}

fn generate_try_new_content_token_stream_9a677220_67b3_4d4d_a7b7_92314cce8e40(
    ident: &dyn quote::ToTokens,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    filter: &dyn std::fmt::Display,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type_ident_where_element_filter_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = &{
        let value = format!(
            "{postgresql_type_or_json_type}{}{}{}",
            &naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident),
            filter,
            naming::TryNewErrorNamedUpperCamelCase
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    generate_try_new_content_token_stream_32f214a7_79ed_4fae_8fdd_0123bd84c652(
        &ident,
        &postgresql_type_or_json_type_ident_where_element_filter_try_new_error_named_upper_camel_case,
    )
}
fn generate_impl_deserialize_token_stream_4b33e130_e350_4911_a82e_0b77a3c433da(
    ident: &dyn quote::ToTokens,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    postgresql_json_array_element_type: &crate::PostgresqlJsonArrayElementType,
    filter: &dyn std::fmt::Display,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type_ident_where_element_position_equals_upper_camel_case: &dyn quote::ToTokens = &{
        let value = format!(
            "{postgresql_type_or_json_type}{}{}",
            &naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident),
            filter,
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    generate_impl_deserialize_token_stream_f8893cac_9ce5_4333_81c7_6bfa8d56c65d(
        &quote::quote!{#postgresql_type_or_json_type_ident_where_element_position_equals_upper_camel_case},
        postgresql_json_array_element_type
    )
}
fn generate_try_generate_bind_increments_token_stream_f50ed702_349c_491f_9f15_cf799072747d(format_handle_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let increment_snake_case = naming::IncrementSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    quote::quote!{
        match #increment_snake_case.checked_add(1) {
            Some(first_increment) => {
                *#increment_snake_case = first_increment;
                match #increment_snake_case.checked_add(1) {
                    Some(second_increment) => {
                        *#increment_snake_case = second_increment;
                        Ok(format!(
                            #format_handle_token_stream,
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            first_increment,
                            second_increment,
                        ))
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
fn generate_bind_value_to_query_token_stream_1026d141_062b_43c0_bbbf_a45d6dfe68a2() -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let position_snake_case = naming::PositionSnakeCase;
    quote::quote!{
        #query_snake_case = #query_snake_case.bind(self.#position_snake_case);
        #query_snake_case = #query_snake_case.bind(sqlx::types::Json(self.#value_snake_case));
        #query_snake_case
    }
}

pub struct PositionEquals;
impl WhereOperatorName for PositionEquals {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::PositionEqualsUpperCamelCase
    }
}
impl PositionEquals {
    fn generate_try_generate_bind_increments_token_stream() -> proc_macro2::TokenStream {
        generate_try_generate_bind_increments_token_stream_f50ed702_349c_491f_9f15_cf799072747d(&quote::quote!{"{}({}->${} = ${})"})
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        postgresql_json_type: &crate::PostgresqlJsonType,
        postgresql_json_array_element_type: &crate::PostgresqlJsonArrayElementType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_json_type));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
        let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_34095bbb_d306_4a44_92e9_4df1a7354bc1(&postgresql_json_array_element_type);
        generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &postgresql_type_or_json_type,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &postgresql_json_type,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_06af1515_1384_4d10_a4cf_aaf07284fd08(),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &generate_try_new_content_token_stream_9a677220_67b3_4d4d_a7b7_92314cce8e40(
                    &postgresql_json_type,
                    &postgresql_type_or_json_type,
                    &self_upper_camel_case,
                ),
                impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_4b33e130_e350_4911_a82e_0b77a3c433da(
                    &postgresql_json_type,
                    &postgresql_type_or_json_type,
                    &postgresql_json_array_element_type,
                    &self_upper_camel_case,
                )
            },
            &crate::ShouldDeriveSchemarsJsonSchema::True,
            &additional_type_declaration_token_stream,
            &generate_additional_default_initialization_token_stream_cd86231b_5ff2_4e91_9906_29b822838309(&{
                let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                    let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
                    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
                    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
                    quote::quote! {
                        crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
                    }
                };
                crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream
            }),
            &Self::generate_try_generate_bind_increments_token_stream(),
            &generate_bind_value_to_query_token_stream_1026d141_062b_43c0_bbbf_a45d6dfe68a2()
        )
    }
}

pub struct PositionGreaterThan;
impl WhereOperatorName for PositionGreaterThan {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::PositionGreaterThanUpperCamelCase
    }
}
impl PositionGreaterThan {
    fn generate_try_generate_bind_increments_token_stream() -> proc_macro2::TokenStream {
        generate_try_generate_bind_increments_token_stream_f50ed702_349c_491f_9f15_cf799072747d(&quote::quote!{"{}({}->${} > ${})"})
    }
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        postgresql_json_type: &crate::PostgresqlJsonType,
        postgresql_json_array_element_type: &crate::PostgresqlJsonArrayElementType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_json_type));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
        let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_34095bbb_d306_4a44_92e9_4df1a7354bc1(&postgresql_json_array_element_type);
        generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &postgresql_type_or_json_type,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            crate::ShouldWhereElementFieldsBePublic::False {
                ident: &postgresql_json_type,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_06af1515_1384_4d10_a4cf_aaf07284fd08(),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &generate_try_new_content_token_stream_9a677220_67b3_4d4d_a7b7_92314cce8e40(
                    &postgresql_json_type,
                    &postgresql_type_or_json_type,
                    &self_upper_camel_case,
                ),
                impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_4b33e130_e350_4911_a82e_0b77a3c433da(
                    &postgresql_json_type,
                    &postgresql_type_or_json_type,
                    &postgresql_json_array_element_type,
                    &self_upper_camel_case,
                )
            },
            &crate::ShouldDeriveSchemarsJsonSchema::True,
            &additional_type_declaration_token_stream,
            &generate_additional_default_initialization_token_stream_cd86231b_5ff2_4e91_9906_29b822838309(&{
                let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                    let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
                    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
                    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
                    quote::quote! {
                        crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
                    }
                };
                crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream
            }),
            &Self::generate_try_generate_bind_increments_token_stream(),
            &generate_bind_value_to_query_token_stream_1026d141_062b_43c0_bbbf_a45d6dfe68a2()
        )
    }
}

//todo this functions used only once coz in the future will be postgresql types support. now its only postgresql json types
fn generate_try_new_content_token_stream_e8513d93_a3ac_4c83_abb1_6f8c8749bcad(
    ident: &dyn quote::ToTokens,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    regular_expression: &RegularExpression,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type_ident_where_element_position_regular_expression_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match (&postgresql_type_or_json_type, &regular_expression) {
        (crate::PostgresqlTypeOrJsonType::PostgresqlType, RegularExpression::CaseSensitive) => &naming::parameter::PostgresqlTypeSelfWhereElementPositionCaseSensitiveRegularExpressionTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        (crate::PostgresqlTypeOrJsonType::PostgresqlJsonType, RegularExpression::CaseSensitive) => &naming::parameter::PostgresqlJsonTypeSelfWhereElementPositionCaseSensitiveRegularExpressionTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        (crate::PostgresqlTypeOrJsonType::PostgresqlType, RegularExpression::CaseInsensitive) => &naming::parameter::PostgresqlTypeSelfWhereElementPositionCaseInsensitiveRegularExpressionTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        (crate::PostgresqlTypeOrJsonType::PostgresqlJsonType, RegularExpression::CaseInsensitive) => &naming::parameter::PostgresqlJsonTypeSelfWhereElementPositionCaseInsensitiveRegularExpressionTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
    };
    generate_try_new_content_token_stream_32f214a7_79ed_4fae_8fdd_0123bd84c652(
        &ident,
        &postgresql_type_or_json_type_ident_where_element_position_regular_expression_try_new_error_named_upper_camel_case,
    )
}
fn generate_impl_deserialize_token_stream_aa92bdde_fcc8_4d46_aae2_d66234bef357(
    ident: &dyn quote::ToTokens,
    postgresql_type_or_json_type: &crate::PostgresqlTypeOrJsonType,
    regular_expression: &RegularExpression,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type_ident_where_element_position_regular_expression_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match (&postgresql_type_or_json_type, &regular_expression) {
        (crate::PostgresqlTypeOrJsonType::PostgresqlType, RegularExpression::CaseSensitive) => &naming::parameter::PostgresqlTypeSelfWhereElementPositionCaseSensitiveRegularExpressionUpperCamelCase::from_tokens(&ident),
        (crate::PostgresqlTypeOrJsonType::PostgresqlJsonType, RegularExpression::CaseSensitive) => &naming::parameter::PostgresqlJsonTypeSelfWhereElementPositionCaseSensitiveRegularExpressionUpperCamelCase::from_tokens(&ident),
        (crate::PostgresqlTypeOrJsonType::PostgresqlType, RegularExpression::CaseInsensitive) => &naming::parameter::PostgresqlTypeSelfWhereElementPositionCaseInsensitiveRegularExpressionUpperCamelCase::from_tokens(&ident),
        (crate::PostgresqlTypeOrJsonType::PostgresqlJsonType, RegularExpression::CaseInsensitive) => &naming::parameter::PostgresqlJsonTypeSelfWhereElementPositionCaseInsensitiveRegularExpressionUpperCamelCase::from_tokens(&ident),
    };
    generate_impl_deserialize_token_stream_f8893cac_9ce5_4333_81c7_6bfa8d56c65d(
        &postgresql_type_or_json_type_ident_where_element_position_regular_expression_upper_camel_case,
        &token_patterns::StdStringString,
    )
}
fn generate_try_generate_bind_increments_token_stream_4334f73e_cfcd_494b_aa9c_54a4aa8a0e39(regular_expression: &RegularExpression) -> proc_macro2::TokenStream {
    let increment_snake_case = naming::IncrementSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let regular_expression_stringified = regular_expression.stringified();
    let format_handle_token_stream = format!("{{}}({{}}->>${{}} ~{regular_expression_stringified} ${{}})");
    quote::quote!{
        match #increment_snake_case.checked_add(1) {
            Some(first_increment) => {
                *#increment_snake_case = first_increment;
                match #increment_snake_case.checked_add(1) {
                    Some(second_increment) => {
                        *#increment_snake_case = second_increment;
                        Ok(format!(
                            #format_handle_token_stream,
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            first_increment,
                            second_increment,
                        ))
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
fn generate_bind_value_to_query_token_stream_32f8f7cf_128f_474a_8e53_f0f0dec3720c() -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let position_snake_case = naming::PositionSnakeCase;
    quote::quote!{
        #query_snake_case = #query_snake_case.bind(self.#position_snake_case);
        #query_snake_case = #query_snake_case.bind(self.#value_snake_case);
        #query_snake_case
    }
}
fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_929ea142_c901_49f1_9334_a57db5aa4b50(
    self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    postgresql_json_type: &crate::PostgresqlJsonType,
    regular_expression: &RegularExpression,
) -> proc_macro2::TokenStream {
    let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
        let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_json_type));
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
    let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_34095bbb_d306_4a44_92e9_4df1a7354bc1(&token_patterns::StdStringString);
    generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
        &postgresql_type_or_json_type,
        &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
        crate::ShouldWhereElementFieldsBePublic::False {
            ident: &postgresql_json_type,
            postfix: &self_upper_camel_case,
            try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_06af1515_1384_4d10_a4cf_aaf07284fd08(),
            try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
            try_new_content_token_stream: &generate_try_new_content_token_stream_e8513d93_a3ac_4c83_abb1_6f8c8749bcad(
                &postgresql_json_type,
                &postgresql_type_or_json_type,
                &regular_expression,
            ),
            impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_aa92bdde_fcc8_4d46_aae2_d66234bef357(
                &postgresql_json_type,
                &postgresql_type_or_json_type,
                &regular_expression,
            )
        },
        &crate::ShouldDeriveSchemarsJsonSchema::True,
        &additional_type_declaration_token_stream,
        &generate_additional_default_initialization_token_stream_cd86231b_5ff2_4e91_9906_29b822838309(&token_patterns::CoreDefaultDefaultDefault),
        &generate_try_generate_bind_increments_token_stream_4334f73e_cfcd_494b_aa9c_54a4aa8a0e39(&regular_expression),
        &generate_bind_value_to_query_token_stream_32f8f7cf_128f_474a_8e53_f0f0dec3720c(),
    )
}

pub struct PositionCaseSensitiveRegularExpression;
impl WhereOperatorName for PositionCaseSensitiveRegularExpression {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::PositionCaseSensitiveRegularExpressionUpperCamelCase
    }
}
impl PositionCaseSensitiveRegularExpression {
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        postgresql_json_type: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_929ea142_c901_49f1_9334_a57db5aa4b50(
            &WhereOperatorName::upper_camel_case(self),
            &postgresql_json_type,
            &RegularExpression::CaseSensitive,
        )
    }
}

pub struct PositionCaseInsensitiveRegularExpression;
impl WhereOperatorName for PositionCaseInsensitiveRegularExpression {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::PositionCaseInsensitiveRegularExpressionUpperCamelCase
    }
}
impl PositionCaseInsensitiveRegularExpression {
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        postgresql_json_type: &crate::PostgresqlJsonType,
    ) -> proc_macro2::TokenStream {
        generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_929ea142_c901_49f1_9334_a57db5aa4b50(
            &WhereOperatorName::upper_camel_case(self),
            &postgresql_json_type,
            &RegularExpression::CaseInsensitive,
        )
    }
}

fn generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(format_handle_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let element_snake_case = naming::ElementSnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let acc_snake_case = naming::AccSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    quote::quote!{
        match #increment_snake_case.checked_add(1) {
            Some(#value_snake_case) => {
                *#increment_snake_case = #value_snake_case;
                Ok(format!(
                    #format_handle_token_stream,
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
fn generate_bind_value_to_query_token_stream_575c4454_6fbb_42c6_bc9c_5bb55c954f1b() -> proc_macro2::TokenStream {
    let element_snake_case = naming::ElementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote!{
        #query_snake_case = #query_snake_case.bind(sqlx::types::Json(self.#value_snake_case));
        #query_snake_case
    }
}
fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_4b900587_aaed_486e_ab9c_e686ae58e5f8(
    self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    postgresql_json_type: &crate::PostgresqlJsonType,
    postgresql_json_array_element_type: &crate::PostgresqlJsonArrayElementType,
    format_handle_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type = crate::PostgresqlTypeOrJsonType::PostgresqlJsonType;
    let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
        let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_json_type));
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_0bec460b_9b9e_46ba_9b8f_42392c528735(&postgresql_json_array_element_type);
    generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
        &postgresql_type_or_json_type,
        &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
        crate::ShouldWhereElementFieldsBePublic::False {
            ident: &postgresql_json_type,
            postfix: &self_upper_camel_case,
            try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_c7fb6522_d391_4af8_b39f_9c6611d199c5(&postgresql_json_array_element_type),
            try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
            try_new_content_token_stream: &generate_try_new_content_token_stream_83fd9d8a_51ab_4c3c_97fc_be2258f730fc(
                &postgresql_json_type,
                &postgresql_type_or_json_type,
                &self_upper_camel_case,
            ),
            impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_45e71974_8062_4f69_9296_248692666dde(
                &postgresql_json_type,
                &postgresql_type_or_json_type,
                &postgresql_json_array_element_type,
                &self_upper_camel_case,
            ),
        },
        &crate::ShouldDeriveSchemarsJsonSchema::True,
        &additional_type_declaration_token_stream,
        &generate_additional_default_initialization_token_stream_e46171d1_2c28_450b_85a8_80293ebe69f8(&{
            let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
                let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
                let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
                quote::quote! {
                    crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
                }
            };
            crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream
        }),
        &generate_try_generate_bind_increments_token_stream_cc8c69fa_8d39_425e_8875_201168042b0a(&format_handle_token_stream),
        &generate_bind_value_to_query_token_stream_575c4454_6fbb_42c6_bc9c_5bb55c954f1b(),
    )
}

pub struct ContainsAllElementsOfArray;
impl WhereOperatorName for ContainsAllElementsOfArray {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::ContainsAllElementsOfArrayUpperCamelCase
    }
}
impl ContainsAllElementsOfArray {
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        postgresql_json_type: &crate::PostgresqlJsonType,
        postgresql_json_array_element_type: &crate::PostgresqlJsonArrayElementType,
    ) -> proc_macro2::TokenStream {
        generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_4b900587_aaed_486e_ab9c_e686ae58e5f8(
            WhereOperatorName::upper_camel_case(self),
            postgresql_json_type,
            postgresql_json_array_element_type,
            &quote::quote!{"{}({} @> ${})"},
        )
    }
}

pub struct ContainedInArray;
impl WhereOperatorName for ContainedInArray {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::ContainedInArrayUpperCamelCase
    }
}
impl ContainedInArray {
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        postgresql_json_type: &crate::PostgresqlJsonType,
        postgresql_json_array_element_type: &crate::PostgresqlJsonArrayElementType,
    ) -> proc_macro2::TokenStream {
        generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_4b900587_aaed_486e_ab9c_e686ae58e5f8(
            WhereOperatorName::upper_camel_case(self),
            postgresql_json_type,
            postgresql_json_array_element_type,
            &quote::quote!{"{}({} <@ ${})"},
        )
    }
}

pub struct OverlapsWithArray;
impl WhereOperatorName for OverlapsWithArray {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::OverlapsWithArrayUpperCamelCase
    }
}
impl OverlapsWithArray {
    pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        postgresql_json_type: &crate::PostgresqlJsonType,
        postgresql_json_array_element_type: &crate::PostgresqlJsonArrayElementType,
    ) -> proc_macro2::TokenStream {
        generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_4b900587_aaed_486e_ab9c_e686ae58e5f8(
            WhereOperatorName::upper_camel_case(self),
            postgresql_json_type,
            postgresql_json_array_element_type,
            &quote::quote!{"{}(exists (select 1 from jsonb_array_elements_text({}) as e1 join jsonb_array_elements_text(${}) as e2 on e1.value = e2.value))"},
        )
    }
}