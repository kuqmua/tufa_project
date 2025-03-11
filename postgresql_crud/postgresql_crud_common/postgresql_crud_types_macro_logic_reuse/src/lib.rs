mod filters;

fn generate_pub_struct_tokens_token_stream(ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens, impl_default: std::primitive::bool, impl_deserialize: std::primitive::bool) -> proc_macro2::TokenStream {
    let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
    let maybe_impl_default_token_stream: &dyn quote::ToTokens = if impl_default { &quote::quote! {Default,} } else { &proc_macro2_token_stream_new };
    let maybe_impl_serde_deserialize_token_stream: &dyn quote::ToTokens = if impl_deserialize { &quote::quote! {serde::Deserialize,} } else { &proc_macro2_token_stream_new };
    quote::quote! {
        #[derive(
            Debug,
            #maybe_impl_default_token_stream
            Clone,
            PartialEq,
            serde::Serialize,
            #maybe_impl_serde_deserialize_token_stream
        )]
        pub struct #ident_token_stream #content_token_stream
    }
}
fn generate_impl_std_fmt_display_for_tokens_token_stream(ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let self_snake_case = naming::SelfSnakeCase;
    quote::quote! {
        impl std::fmt::Display for #ident_token_stream {
            fn fmt(&#self_snake_case, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, #content_token_stream)
            }
        }
    }
}
fn generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let error_occurence_lib_snake_case = naming::ErrorOccurenceLibSnakeCase;
    let to_std_string_string_upper_camel_case = naming::ToStdStringStringUpperCamelCase;
    let to_std_string_string_snake_case = naming::ToStdStringStringSnakeCase;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let self_snake_case = naming::SelfSnakeCase;
    quote::quote! {
        impl #error_occurence_lib_snake_case::#to_std_string_string_upper_camel_case for #ident {
            fn #to_std_string_string_snake_case(&#self_snake_case) -> #std_string_string_token_stream {
                #content_token_stream
            }
        }
    }
}
enum PostgresqlTypeOrJsonType {
    PostgresqlType,
    PostgresqlJsonType,
}
impl std::fmt::Display for PostgresqlTypeOrJsonType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::PostgresqlType => write!(formatter, "{}", naming::PostgresqlTypeUpperCamelCase),
            Self::PostgresqlJsonType => write!(formatter, "{}", naming::PostgresqlJsonTypeUpperCamelCase),
        }
    }
}
enum ShouldDeriveSchemarsJsonSchema {
    True,
    False,
}
impl std::convert::From<&PostgresqlTypeOrJsonType> for ShouldDeriveSchemarsJsonSchema {
    fn from(value: &PostgresqlTypeOrJsonType) -> Self {
        match &value {
            PostgresqlTypeOrJsonType::PostgresqlType => Self::False,
            PostgresqlTypeOrJsonType::PostgresqlJsonType => Self::True,
        }
    }
}
impl quote::ToTokens for ShouldDeriveSchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {, schemars::JsonSchema}.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}
fn generate_postgresql_type_or_json_type_where_token_stream(
    postgresql_type_or_json_type: &PostgresqlTypeOrJsonType,
    ident: &dyn quote::ToTokens,
    postgresql_type_or_json_type_ident_where_element_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    postgresql_type_or_json_type_ident_where_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
) -> proc_macro2::TokenStream {
    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = token_patterns::CrateGeneratePostgresqlJsonTypeStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;
    let postgresql_type_or_json_type_ident_where_token_stream = {
        let should_derive_schemars_json_schema = ShouldDeriveSchemarsJsonSchema::from(postgresql_type_or_json_type);
        quote::quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize #should_derive_schemars_json_schema)]
            pub struct #postgresql_type_or_json_type_ident_where_upper_camel_case {
                logical_operator: crate::LogicalOperator,
                value: std::vec::Vec<#postgresql_type_or_json_type_ident_where_element_upper_camel_case>,
            }
        }
    };
    let postgresql_type_or_json_type_ident_where_try_new_error_named_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_or_json_type {
        PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
    };
    let is_empty_upper_camel_case = naming::IsEmptyUpperCamelCase;
    let not_unique_upper_camel_case = naming::NotUniqueUpperCamelCase;
    let postgresql_type_or_json_type_ident_where_try_new_error_named_token_stream = {
        quote::quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum #postgresql_type_or_json_type_ident_where_try_new_error_named_upper_camel_case {
                #is_empty_upper_camel_case {
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                #not_unique_upper_camel_case {
                    #[eo_to_std_string_string_serialize_deserialize]
                    value: #postgresql_type_or_json_type_ident_where_element_upper_camel_case,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
            }
        }
    };
    let impl_postgresql_type_or_json_type_ident_where_try_new_token_stream = {
        let logical_operator_upper_camel_case = naming::LogicalOperatorUpperCamelCase;
        let logical_operator_snake_case = naming::LogicalOperatorSnakeCase;
        quote::quote! {
            impl #postgresql_type_or_json_type_ident_where_upper_camel_case {
                fn try_new(
                    #logical_operator_snake_case: crate::#logical_operator_upper_camel_case,
                    value: std::vec::Vec<#postgresql_type_or_json_type_ident_where_element_upper_camel_case>,
                ) -> Result<Self, #postgresql_type_or_json_type_ident_where_try_new_error_named_upper_camel_case> {
                    if value.is_empty() {
                        return Err(#postgresql_type_or_json_type_ident_where_try_new_error_named_upper_camel_case::#is_empty_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                    {
                        let mut acc = vec![];
                        for element in &value {
                            if !acc.contains(&element) {
                                acc.push(element);
                            } else {
                                return Err(#postgresql_type_or_json_type_ident_where_try_new_error_named_upper_camel_case::#not_unique_upper_camel_case {
                                    value: element.clone(),
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        }
                    }
                    Ok(Self {
                        #logical_operator_snake_case,
                        value,
                    })
                }
            }
        }
    };
    let impl_serde_deserialize_for_postgresql_type_or_json_type_ident_where_token_stream = {
        let (struct_postgresql_json_type_ident_where_double_quotes_token_stream, struct_postgresql_json_type_ident_where_with_2_elements_double_quotes_token_stream, postgresql_json_type_ident_where_double_quotes_token_stream) =
            crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_upper_camel_case, 2);
        quote::quote! {
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for #postgresql_type_or_json_type_ident_where_upper_camel_case {
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
                            marker: _serde::__private::PhantomData<#postgresql_type_or_json_type_ident_where_upper_camel_case>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #postgresql_type_or_json_type_ident_where_upper_camel_case;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    #struct_postgresql_json_type_ident_where_double_quotes_token_stream,
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
                                                &#struct_postgresql_json_type_ident_where_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    std::vec::Vec<#postgresql_type_or_json_type_ident_where_element_upper_camel_case>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &#struct_postgresql_json_type_ident_where_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                match #postgresql_type_or_json_type_ident_where_upper_camel_case::try_new(__field0, __field1) {
                                    Ok(value) => serde::__private::Ok(value),
                                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
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
                                let mut __field1: _serde::__private::Option<std::vec::Vec<#postgresql_type_or_json_type_ident_where_element_upper_camel_case>> = _serde::__private::None;
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
                                                    std::vec::Vec<#postgresql_type_or_json_type_ident_where_element_upper_camel_case>,
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
                                match #postgresql_type_or_json_type_ident_where_upper_camel_case::try_new(__field0, __field1) {
                                    Ok(value) => serde::__private::Ok(value),
                                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
                                }
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #postgresql_json_type_ident_where_double_quotes_token_stream,
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<#postgresql_type_or_json_type_ident_where_upper_camel_case>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
        }
    };
    let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_or_json_type_ident_where_token_stream = {
        quote::quote! {
            impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_or_json_type_ident_where_upper_camel_case {
                fn self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                    let mut acc = std::string::String::default();
                    let mut is_need_to_add_logical_operator_inner_handle = false;
                    for element in &self.value {
                        match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::self_where_try_generate_bind_increments(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                            Ok(value) => {
                                acc.push_str(&format!("{value} "));
                                is_need_to_add_logical_operator_inner_handle = true;
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        }
                    }
                    let _ = acc.pop();
                    Ok(format!("{}({acc})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                }
                fn self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    for element in self.value {
                        query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::self_where_bind_value_to_query(element, query);
                    }
                    query
                }
            }
        }
    };
    let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_or_json_type_ident_where_token_stream =
        postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(&postgresql_type_or_json_type_ident_where_upper_camel_case, &{
            let crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream =
                token_patterns::CrateGeneratePostgresqlJsonTypeAllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;
            quote::quote! {
                Self {
                    logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    value: #crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                }
            }
        });
    quote::quote! {
        #postgresql_type_or_json_type_ident_where_token_stream
        #postgresql_type_or_json_type_ident_where_try_new_error_named_token_stream
        #impl_postgresql_type_or_json_type_ident_where_try_new_token_stream
        #impl_serde_deserialize_for_postgresql_type_or_json_type_ident_where_token_stream
        #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_or_json_type_ident_where_token_stream
        #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_or_json_type_ident_where_token_stream
    }
}
fn generate_impl_sqlx_encode_sqlx_postgres_for_tokens_token_stream(ident_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let self_snake_case = naming::SelfSnakeCase;
    quote::quote! {
        impl sqlx::Encode<'_, sqlx::Postgres> for #ident_token_stream {
            fn encode_by_ref(&#self_snake_case, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#self_snake_case.0, buf)
            }
        }
    }
}
fn generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(ident_token_stream: &dyn quote::ToTokens, field_type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let error_snake_case = naming::ErrorSnakeCase;
    quote::quote! {
        impl sqlx::Decode<'_, sqlx::Postgres> for #ident_token_stream {
            fn decode(#value_snake_case: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                match <#field_type_token_stream as sqlx::Decode<sqlx::Postgres>>::decode(#value_snake_case) {
                    Ok(#value_snake_case) => Ok(Self(#value_snake_case)),
                    Err(#error_snake_case) => Err(#error_snake_case)
                }
            }
        }
    }
}
fn generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(ident_token_stream: &dyn quote::ToTokens, field_type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {
        impl sqlx::Type<sqlx::Postgres> for #ident_token_stream {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
               <#field_type_token_stream as sqlx::Type<sqlx::Postgres>>::type_info()
            }
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
                <#field_type_token_stream as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }
    }
}
fn crate_try_generate_bind_increments_error_named_token_stream() -> proc_macro2::TokenStream {
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    quote::quote! {crate::#try_generate_bind_increments_error_named_upper_camel_case}
}
fn crate_bind_query_token_stream() -> proc_macro2::TokenStream {
    quote::quote! {crate::BindQuery::}
}
fn generate_impl_crate_bind_query_for_tokens_token_stream(ident_token_stream: &dyn quote::ToTokens, try_generate_bind_increments_token_stream: &dyn quote::ToTokens, bind_value_to_query_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let std_string_string_token_stream = token_patterns::StdStringString;
    let self_snake_case = naming::SelfSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let crate_bind_query_token_stream = crate_bind_query_token_stream();
    let try_generate_bind_increments_snake_case = naming::TryGenerateBindIncrementsSnakeCase;
    let bind_value_to_query_snake_case = naming::BindValueToQuerySnakeCase;
    let crate_try_generate_bind_increments_error_named_token_stream = crate_try_generate_bind_increments_error_named_token_stream();
    quote::quote! {
        impl #crate_bind_query_token_stream<'_> for #ident_token_stream {
            fn #try_generate_bind_increments_snake_case(&#self_snake_case, #increment_snake_case: &mut std::primitive::u64) -> Result<#std_string_string_token_stream, #crate_try_generate_bind_increments_error_named_token_stream> {
                #try_generate_bind_increments_token_stream
            }
            fn #bind_value_to_query_snake_case(#self_snake_case, mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #bind_value_to_query_token_stream
            }
        }
    }
}
fn generate_postgresql_type_where_element_token_stream(
    variants: &std::vec::Vec<&dyn crate::filters::WhereOperatorName>,
    ident: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    variant_type_prefix_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    should_implement_schemars_json_schema: &ShouldDeriveSchemarsJsonSchema,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let postgresql_type_tokens_where_element_token_stream = {
        let variants_token_stream = variants.iter().map(|element| {
            let element_upper_camel_case = element.upper_camel_case();
            let postgresql_type_tokens_where_element_filter_upper_camel_case = {
                let value = format!("{variant_type_prefix_upper_camel_case}{}", quote::quote! {#element_upper_camel_case});
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote! {#element_upper_camel_case(#postgresql_type_tokens_where_element_filter_upper_camel_case)}
        });
        quote::quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_implement_schemars_json_schema)]
            pub enum #ident {
                #(#variants_token_stream),*
            }
        }
    };
    let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_tokens_where_element_token_stream = generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_token_stream(
        &ident,
        &{
            let variants_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.upper_camel_case();
                quote::quote! {
                    Self::#element_upper_camel_case(#value_snake_case) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::self_where_try_generate_bind_increments(
                        #value_snake_case,
                        #increment_snake_case,
                        #column_snake_case,
                        #is_need_to_add_logical_operator_snake_case,
                    )
                }
            });
            quote::quote! {
                match &self {
                    #(#variants_token_stream),*
                }
            }
        },
        &{
            let variants_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.upper_camel_case();
                quote::quote! {
                    Self::#element_upper_camel_case(#value_snake_case) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::self_where_bind_value_to_query(
                        #value_snake_case,
                        #query_snake_case
                    )
                }
            });
            quote::quote! {
                match self {
                    #(#variants_token_stream),*
                }
            }
        },
    );
    let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(&ident, &quote::quote! {format!("{self:#?}")});
    let impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_tokens_where_element_token_stream =
        postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(&ident, &{
            let variants_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.upper_camel_case();
                let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream =
                    token_patterns::CrateGeneratePostgresqlJsonTypeStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;
                quote::quote! {
                    Self::#element_upper_camel_case(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
                }
            });
            quote::quote! {vec![#(#variants_token_stream),*]}
        });
    quote::quote! {
        #postgresql_type_tokens_where_element_token_stream
        #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_tokens_where_element_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream
        #impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_tokens_where_element_token_stream
    }
}
fn generate_struct_postgresql_type_where_element_tokens_double_quotes_token_stream(postgresql_type_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_token_stream(&format!("struct {postgresql_type_where_element_tokens_upper_camel_case}"))
}
fn generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens, length: std::primitive::usize) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_token_stream(&format!("struct {postgresql_type_ident_where_element_tokens_upper_camel_case} with {length} elements"))
}
fn generate_serde_deserialize_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens, length: std::primitive::usize) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream = generate_struct_postgresql_type_where_element_tokens_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case);
    let struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream = generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case, length);
    let postgresql_type_ident_where_element_tokens_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&postgresql_type_ident_where_element_tokens_upper_camel_case);
    (
        struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream,
        struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream,
        postgresql_type_ident_where_element_tokens_double_quotes_token_stream,
    )
}
fn generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    postgresql_type_self_where_try_generate_bind_increments_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_bind_value_to_query_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let increment_snake_case = naming::IncrementSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    quote::quote! {
        impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #ident {
            fn self_where_try_generate_bind_increments(
                &self,
                #increment_snake_case: &mut std::primitive::u64,
                #column_snake_case : &dyn std::fmt::Display,
                #is_need_to_add_logical_operator_snake_case: std::primitive::bool,
            ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                #postgresql_type_self_where_try_generate_bind_increments_token_stream
            }
            fn self_where_bind_value_to_query<'a>(
                self,
                mut #query_snake_case: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #postgresql_type_self_where_bind_value_to_query_token_stream
            }
        }
    }
}

enum PostgresqlTypeNotNullOrNullable {
    NotNull,
    Nullable,
}

enum WhereOperatorType<'a> {
    Ident(&'a dyn quote::ToTokens),
    FieldType { field_type: &'a dyn quote::ToTokens, default_initialization_token_stream: &'a dyn quote::ToTokens },
}
impl WhereOperatorType<'_> {
    fn type_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(value) => quote::quote! {#value},
            WhereOperatorType::FieldType { field_type, .. } => quote::quote! {#field_type},
        }
    }
    fn std_option_option_type_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(value) => quote::quote! {std::option::Option<#value>},
            WhereOperatorType::FieldType { field_type, .. } => quote::quote! {std::option::Option<#field_type>},
        }
    }
    fn additional_bind_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(_) => quote::quote! {.0},
            WhereOperatorType::FieldType { .. } => proc_macro2::TokenStream::new(),
        }
    }
    fn default_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(_) => {
                let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream =
                    token_patterns::CrateGeneratePostgresqlJsonTypeStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;
                quote::quote! {#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream}
            }
            WhereOperatorType::FieldType { field_type: _, default_initialization_token_stream } => quote::quote! {#default_initialization_token_stream},
        }
    }
    fn std_option_option_default_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(_) => {
                let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream =
                    token_patterns::CrateGeneratePostgresqlJsonTypeStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;
                quote::quote! {Some(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)}
            }
            WhereOperatorType::FieldType { field_type: _, default_initialization_token_stream } => quote::quote! {Some(#default_initialization_token_stream)},
        }
    }
}

struct PostgresqlJsonTypeVariant {
    postgresql_json_type_handle: PostgresqlJsonTypeHandle,
    postgresql_json_type_pattern: PostgresqlJsonTypePattern,
}
impl PostgresqlJsonTypeVariant {
    // fn all_variants() -> std::vec::Vec<Self> {
    //     let mut acc = vec![];
    //     for postgresql_json_type_handle in PostgresqlJsonTypeHandle::into_array() {
    //         for postgresql_json_type_pattern in PostgresqlJsonTypePattern::all_variants() {
    //             acc.push(Self {
    //                 postgresql_json_type_handle: postgresql_json_type_handle.clone(),
    //                 postgresql_json_type_pattern: postgresql_json_type_pattern,
    //             });
    //         }
    //     }
    //     acc
    // }
    fn try_to_vec_element_type(&self) -> Result<Self, ()> {
        match &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
            PostgresqlJsonTypePatternType::FullTypePath => Err(()),
            //todo maybe wrong
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath | PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => Ok(Self {
                postgresql_json_type_handle: self.postgresql_json_type_handle.clone(),
                postgresql_json_type_pattern: self.postgresql_json_type_pattern.clone(),
            }),
        }
    }
    fn postgresql_json_type_ident_wrapper(&self) -> proc_macro2::TokenStream {
        format!("{}{}", self.postgresql_json_type_pattern.prefix_stringified(), self.postgresql_json_type_handle,).parse::<proc_macro2::TokenStream>().unwrap()
    }

    fn handle_field_type(&self, is_wrapper: std::primitive::bool) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &self.postgresql_json_type_handle;
        match (&self.postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::FullTypePath) => {
                if is_wrapper {
                    quote::quote! {#postgresql_json_type_handle}
                } else {
                    postgresql_json_type_handle.field_type_token_stream()
                }
            }
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::FullTypePath) => quote::quote! {std::option::Option<#postgresql_json_type_handle>},
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => quote::quote! {std::vec::Vec<#postgresql_json_type_handle>},

            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => {
                // let value = {
                //     format!("{}{postgresql_json_type_handle}", &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type.prefix_stringified())
                //     .parse::<proc_macro2::TokenStream>()
                //     .unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonTypeHandle to proc_macro2::TokenStream"))
                // };
                // quote::quote!{std::option::Option<#value>}
                quote::quote! {std::option::Option<std::vec::Vec<#postgresql_json_type_handle>>}
            }
            (&PostgresqlJsonTypePatternIsOptional::False, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote! {std::vec::Vec<std::vec::Vec<#postgresql_json_type_handle>>}
            }
            (&PostgresqlJsonTypePatternIsOptional::True, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote! {std::option::Option<std::vec::Vec<std::vec::Vec<#postgresql_json_type_handle>>>}
            }
        }
    }
    fn handle_initialization_token_stream(&self, is_wrapper: std::primitive::bool) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &self.postgresql_json_type_handle;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream =
            token_patterns::CrateGeneratePostgresqlJsonTypeStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;
        match (&self.postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::FullTypePath) => {
                if is_wrapper {
                    quote::quote! {#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream}
                } else {
                    postgresql_json_type_handle.full_type_path_initialization_token_stream()
                }
            }
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::FullTypePath) => quote::quote! {Some(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)},
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => quote::quote! {vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]},
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => {
                quote::quote! {Some(vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream])}
                // quote::quote!{Some(#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)}
            }
            (&PostgresqlJsonTypePatternIsOptional::False, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote! {vec![vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]]}
            }
            (&PostgresqlJsonTypePatternIsOptional::True, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote! {Some(vec![vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]])}
            }
        }
    }
    fn field_type(&self) -> proc_macro2::TokenStream {
        self.handle_field_type(false)
    }
    fn initialization_token_stream(&self) -> proc_macro2::TokenStream {
        self.handle_initialization_token_stream(false)
    }

    fn wrapper_field_type(&self) -> proc_macro2::TokenStream {
        self.handle_field_type(true)
    }
    fn wrapper_non_optional_field_type(&self) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &self.postgresql_json_type_handle;
        match &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
            PostgresqlJsonTypePatternType::FullTypePath => quote::quote! {#postgresql_json_type_handle},
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath => quote::quote! {std::vec::Vec<#postgresql_json_type_handle>},
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => quote::quote! {std::vec::Vec<std::vec::Vec<#postgresql_json_type_handle>>},
        }
    }
    fn wrapper_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        self.handle_initialization_token_stream(true)
    }
    fn wrapper_non_optional_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream =
            token_patterns::CrateGeneratePostgresqlJsonTypeStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;
        match &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
            PostgresqlJsonTypePatternType::FullTypePath => quote::quote! {#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream},
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath => quote::quote! {vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]},
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => quote::quote! {vec![vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream]]},
        }
    }
}

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
enum PostgresqlJsonTypeHandle {
    StdPrimitiveI8,
    StdPrimitiveI16,
    StdPrimitiveI32,
    StdPrimitiveI64,
    StdPrimitiveU8,
    StdPrimitiveU16,
    StdPrimitiveU32,
    StdPrimitiveU64,
    StdPrimitiveF32,
    StdPrimitiveF64,
    StdPrimitiveBool,
    StdStringString,
    UuidUuid,
}
impl PostgresqlJsonTypeHandle {
    fn field_type_stringified(&self) -> &'static std::primitive::str {
        match &self {
            Self::StdPrimitiveI8 => "std::primitive::i8",
            Self::StdPrimitiveI16 => "std::primitive::i16",
            Self::StdPrimitiveI32 => "std::primitive::i32",
            Self::StdPrimitiveI64 => "std::primitive::i64",
            Self::StdPrimitiveU8 => "std::primitive::u8",
            Self::StdPrimitiveU16 => "std::primitive::u16",
            Self::StdPrimitiveU32 => "std::primitive::u32",
            Self::StdPrimitiveU64 => "std::primitive::u64",
            Self::StdPrimitiveF32 => "std::primitive::f32",
            Self::StdPrimitiveF64 => "std::primitive::f64",
            Self::StdPrimitiveBool => "std::primitive::bool",
            Self::StdStringString => "std::string::String",
            Self::UuidUuid => "uuid::Uuid",
        }
    }
    fn field_type_token_stream(&self) -> proc_macro2::TokenStream {
        self.field_type_stringified().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonTypeHandle to proc_macro2::TokenStream"))
    }
    fn full_type_path_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            Self::StdPrimitiveI8
            | Self::StdPrimitiveI16
            | Self::StdPrimitiveI32
            | Self::StdPrimitiveI64
            | Self::StdPrimitiveU8
            | Self::StdPrimitiveU16
            | Self::StdPrimitiveU32
            | Self::StdPrimitiveU64
            | Self::StdPrimitiveF32
            | Self::StdPrimitiveF64
            | Self::StdPrimitiveBool
            | Self::StdStringString => {
                let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
                quote::quote! {#core_default_default_default_token_stream}
            }
            Self::UuidUuid => quote::quote! {
                uuid::Uuid::new_v4()
            },
        }
    }
}
impl quote::ToTokens for PostgresqlJsonTypeHandle {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.to_string().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonTypeHandle to proc_macro2::TokenStream")).to_tokens(tokens)
    }
}

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
enum PostgresqlJsonTypePatternIsOptional {
    True,
    False,
}
impl PostgresqlJsonTypePatternIsOptional {
    fn prefix_stringified(&self) -> std::string::String {
        match &self {
            Self::True => std::string::String::from("StdOptionOption"),
            Self::False => std::string::String::default(),
        }
    }
}

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
enum PostgresqlJsonTypePatternType {
    FullTypePath,
    StdVecVecFullTypePath,
    StdVecVecStdVecVecFullTypePath,
}
impl PostgresqlJsonTypePatternType {
    fn prefix_stringified(&self) -> std::string::String {
        match &self {
            PostgresqlJsonTypePatternType::FullTypePath => std::string::String::default(),
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath => naming::StdVecVecUpperCamelCase.to_string(),
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => naming::StdVecVecStdVecVecUpperCamelCase.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct PostgresqlJsonTypePattern {
    postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional,
    postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType,
}
impl PostgresqlJsonTypePattern {
    fn prefix_stringified(&self) -> std::string::String {
        format!("{}{}", &self.postgresql_json_type_pattern_is_optional.prefix_stringified(), &self.postgresql_json_type_pattern_type.prefix_stringified(),)
    }
    // fn all_variants() -> std::vec::Vec<Self> {
    //     let mut acc = vec![];
    //     for postgresql_json_type_pattern_type in PostgresqlJsonTypePatternType::into_array() {
    //         for postgresql_json_type_pattern_is_optional in PostgresqlJsonTypePatternIsOptional::into_array() {
    //             acc.push(Self {
    //                 postgresql_json_type_pattern_is_optional: postgresql_json_type_pattern_is_optional,
    //                 postgresql_json_type_pattern_type: postgresql_json_type_pattern_type.clone(),
    //             });
    //         }
    //     }
    //     acc
    // }
}

#[proc_macro]
pub fn generate_postgresql_json_types(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    fn generate_postgresql_json_type_handle_token_stream(postgresql_json_type_variant: &PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &postgresql_json_type_variant.postgresql_json_type_handle;
        let postgresql_json_type_pattern = &postgresql_json_type_variant.postgresql_json_type_pattern;
        let postgresql_json_type_ident_wrapper = postgresql_json_type_variant.postgresql_json_type_ident_wrapper();

        let ident: &dyn quote::ToTokens = &postgresql_json_type_ident_wrapper;
        let field_type = &postgresql_json_type_variant.field_type();

        let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();

        let ident_token_stream = {
            let schemars_json_schema_token_stream = quote::quote! {schemars::JsonSchema,};
            let maybe_derive_schemars_json_schema_token_stream: &dyn quote::ToTokens = {
                match (&postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
                    (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::FullTypePath) => match &postgresql_json_type_handle {
                        PostgresqlJsonTypeHandle::StdPrimitiveI8
                        | PostgresqlJsonTypeHandle::StdPrimitiveI16
                        | PostgresqlJsonTypeHandle::StdPrimitiveI32
                        | PostgresqlJsonTypeHandle::StdPrimitiveI64
                        | PostgresqlJsonTypeHandle::StdPrimitiveU8
                        | PostgresqlJsonTypeHandle::StdPrimitiveU16
                        | PostgresqlJsonTypeHandle::StdPrimitiveU32
                        | PostgresqlJsonTypeHandle::StdPrimitiveU64
                        | PostgresqlJsonTypeHandle::StdPrimitiveF32
                        | PostgresqlJsonTypeHandle::StdPrimitiveF64
                        | PostgresqlJsonTypeHandle::StdPrimitiveBool
                        | PostgresqlJsonTypeHandle::StdStringString => &schemars_json_schema_token_stream,
                        PostgresqlJsonTypeHandle::UuidUuid => &proc_macro2_token_stream_new,
                    },
                    (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::FullTypePath)
                    | (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
                    | (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
                    | (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath)
                    | (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => &schemars_json_schema_token_stream,
                }
            };
            quote::quote! {
                #[derive(
                    Debug,
                    Clone,
                    PartialEq,
                    PartialOrd,
                    Default,
                    serde::Serialize,
                    serde::Deserialize,
                    utoipa::ToSchema,
                    #maybe_derive_schemars_json_schema_token_stream
                )]
                pub struct #ident(pub #field_type);//todo #[validate(range(min = -128i8, max = 127i8))]
            }
        };
        let maybe_impl_schemars_json_schema_for_ident_token_stream: &dyn quote::ToTokens = match (&postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::FullTypePath) => match &postgresql_json_type_handle {
                PostgresqlJsonTypeHandle::StdPrimitiveI8
                | PostgresqlJsonTypeHandle::StdPrimitiveI16
                | PostgresqlJsonTypeHandle::StdPrimitiveI32
                | PostgresqlJsonTypeHandle::StdPrimitiveI64
                | PostgresqlJsonTypeHandle::StdPrimitiveU8
                | PostgresqlJsonTypeHandle::StdPrimitiveU16
                | PostgresqlJsonTypeHandle::StdPrimitiveU32
                | PostgresqlJsonTypeHandle::StdPrimitiveU64
                | PostgresqlJsonTypeHandle::StdPrimitiveF32
                | PostgresqlJsonTypeHandle::StdPrimitiveF64
                | PostgresqlJsonTypeHandle::StdPrimitiveBool
                | PostgresqlJsonTypeHandle::StdStringString => &proc_macro2_token_stream_new,
                PostgresqlJsonTypeHandle::UuidUuid => &{
                    let uuid_uuid_upper_camel_case = naming::UuidUuidUpperCamelCase;
                    let schema_name_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&uuid_uuid_upper_camel_case.to_string());
                    let schema_id_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("postgresql_crud_common::f::{uuid_uuid_upper_camel_case}"));
                    quote::quote! {
                        impl schemars::JsonSchema for #uuid_uuid_upper_camel_case {
                            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                schemars::_private::alloc::borrow::Cow::Borrowed(#schema_name_format_handle_token_stream)
                            }
                            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                schemars::_private::alloc::borrow::Cow::Borrowed(#schema_id_format_handle_token_stream)
                            }
                            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                                {
                                    let mut schema = generator.subschema_for::<std::string::String>();
                                    schemars::_private::insert_validation_property(&mut schema, "string", "minLength", 36);
                                    schemars::_private::insert_validation_property(&mut schema, "string", "maxLength", 36);
                                    schemars::_private::insert_validation_property(&mut schema, "array", "minItems", 36);
                                    schemars::_private::insert_validation_property(&mut schema, "array", "maxItems", 36);
                                    schema
                                }
                            }
                        }
                    }
                },
            },
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::FullTypePath)
            | (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
            | (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
            | (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath)
            | (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => &proc_macro2_token_stream_new,
        };

        //todo migrate to newest version of schemars crate then write validation logic.

        // pub struct StdPrimitiveI8(#[validate(range(min = -128i8, max = 127i8))] pub std::primitive::i8);
        // pub struct StdPrimitiveI16(#[validate(range(min = -32_768i16, max = 32_767i16))] pub std::primitive::i16);
        // pub struct StdPrimitiveI32(#[validate(range(min = -2_147_483_648i32, max = 2_147_483_647i32))] pub std::primitive::i32);
        // pub struct StdPrimitiveI64(#[validate(range(min = -9_223_372_036_854_775_808i64, max = 9_223_372_036_854_775_807i64))] pub std::primitive::i64);
        // pub struct StdPrimitiveU8(#[validate(range(min = 0u8, max = 255u8))] pub std::primitive::u8);
        // pub struct StdPrimitiveU16(#[validate(range(min = 0u16, max = 65_535u16))] pub std::primitive::u16);
        // pub struct StdPrimitiveU32(#[validate(range(min = 0u32, max = 4_294_967_295u32))] pub std::primitive::u32);
        // pub struct StdPrimitiveU64(#[validate(range(min = 0u64, max = 18_446_744_073_709_551_615u64))] pub std::primitive::u64);
        // //todo maybe its not correct. https://doc.rust-lang.org/std/primitive.f32.html
        // pub struct StdPrimitiveF32(#[validate(range(min = -3.40282347E+38f32, max = 3.40282347E+38f32))] pub std::primitive::f32);
        // //todo maybe its not correct. https://doc.rust-lang.org/core/primitive.f64.html
        // pub struct StdPrimitiveF64(#[validate(range(min = -1.7976931348623157E+308f64, max = 1.7976931348623157E+308f64))] pub std::primitive::f64);

        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream =
            postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(&ident, &{
                let content_token_stream = postgresql_json_type_variant.initialization_token_stream();
                quote::quote! {
                    Self(#content_token_stream)
                }
            });
        let impl_error_occurence_lib_to_std_string_string_for_ident_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(&ident, &quote::quote! {format!("{self:#?}")});

        let postgresql_json_type_ident_to_create_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfToCreateUpperCamelCase::from_tokens(&ident);
        let postgresql_json_type_ident_to_create_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_json_type_ident_to_create_upper_camel_case, &ident);
        let postgresql_json_type_ident_field_reader_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfFieldReaderUpperCamelCase::from_tokens(&ident);
        let postgresql_json_type_ident_field_reader_token_stream = {
            let content_token_stream = match &postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                PostgresqlJsonTypePatternType::FullTypePath => quote::quote! {{}},
                PostgresqlJsonTypePatternType::StdVecVecFullTypePath => quote::quote! {{ pagination: crate::pagination::Pagination }},
                PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => quote::quote! {{ pagination: crate::pagination::Pagination }}, //todo another pagination?
            };
            quote::quote! {
                #[derive(
                    Debug,
                    Clone,
                    PartialEq,
                    Default,
                    serde::Serialize,
                    serde::Deserialize,
                    utoipa::ToSchema,
                    schemars::JsonSchema,
                )]
                pub struct #postgresql_json_type_ident_field_reader_upper_camel_case #content_token_stream
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream =
            postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(&postgresql_json_type_ident_field_reader_upper_camel_case, &{
                let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream =
                    token_patterns::CrateGeneratePostgresqlJsonTypeStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;
                match &postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                    PostgresqlJsonTypePatternType::FullTypePath => {
                        let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                        quote::quote! {
                            #core_default_default_default
                        }
                    }
                    PostgresqlJsonTypePatternType::StdVecVecFullTypePath => {
                        quote::quote! {
                            Self {
                                pagination: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            }
                        }
                    }
                    PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => {
                        quote::quote! {
                            Self {
                                pagination: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            }
                        }
                    }
                }
            });
        let postgresql_json_type_ident_options_to_read_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfOptionsToReadUpperCamelCase::from_tokens(&ident);
        let postgresql_json_type_ident_options_to_read_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_json_type_ident_options_to_read_upper_camel_case, &ident);

        let postgresql_json_type_ident_where_element_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&ident);
        let postgresql_json_type_ident_where_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfWhereUpperCamelCase::from_tokens(&ident);
        let postgresql_json_type_where_token_stream = generate_postgresql_type_or_json_type_where_token_stream(&PostgresqlTypeOrJsonType::PostgresqlJsonType, &ident, &postgresql_json_type_ident_where_element_upper_camel_case, &postgresql_json_type_ident_where_upper_camel_case);
        enum MaybePostgresqlJsonTypeIdentWhereElementFilter<'a> {
            Some { where_operator_name: &'a dyn crate::filters::WhereOperatorName, token_stream: proc_macro2::TokenStream },
            None,
        }
        impl quote::ToTokens for MaybePostgresqlJsonTypeIdentWhereElementFilter<'_> {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match &self {
                    Self::Some { where_operator_name: _, token_stream } => token_stream.to_tokens(tokens),
                    Self::None => proc_macro2::TokenStream::new().to_tokens(tokens),
                }
            }
        }
        #[derive(Debug, Clone)]
        enum PostgresqlJsonTypeSpecific {
            Number,
            Bool,
            String,
        }
        impl std::convert::From<&PostgresqlJsonTypeHandle> for PostgresqlJsonTypeSpecific {
            fn from(value: &PostgresqlJsonTypeHandle) -> Self {
                match value {
                    PostgresqlJsonTypeHandle::StdPrimitiveI8
                    | PostgresqlJsonTypeHandle::StdPrimitiveI16
                    | PostgresqlJsonTypeHandle::StdPrimitiveI32
                    | PostgresqlJsonTypeHandle::StdPrimitiveI64
                    | PostgresqlJsonTypeHandle::StdPrimitiveU8
                    | PostgresqlJsonTypeHandle::StdPrimitiveU16
                    | PostgresqlJsonTypeHandle::StdPrimitiveU32
                    | PostgresqlJsonTypeHandle::StdPrimitiveU64
                    | PostgresqlJsonTypeHandle::StdPrimitiveF32
                    | PostgresqlJsonTypeHandle::StdPrimitiveF64 => Self::Number,
                    PostgresqlJsonTypeHandle::StdPrimitiveBool => Self::Bool,
                    PostgresqlJsonTypeHandle::StdStringString | PostgresqlJsonTypeHandle::UuidUuid => Self::String,
                }
            }
        }
        let postgresql_json_type_specific = PostgresqlJsonTypeSpecific::from(&postgresql_json_type_variant.postgresql_json_type_handle);
        let postgresql_json_type_ident_where_element_token_stream = {
            let postgresql_json_type_ident_where_element_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&ident);

            let equal = crate::filters::Equal;
            let postgresql_json_type_ident_where_element_equal_token_stream = equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);

            let common_postgresql_json_type_filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = vec![&equal];
            let common_postgresql_json_type_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = vec![postgresql_json_type_ident_where_element_equal_token_stream];

            let length_equal = crate::filters::LengthEqual;
            let postgresql_json_type_ident_where_element_length_equal_token_stream = length_equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);
            let length_more_than = crate::filters::LengthMoreThan;
            let postgresql_json_type_ident_where_element_length_more_than_token_stream = length_more_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);

            let position_equal = crate::filters::PositionEqual;
            let position_greater_than = crate::filters::PositionGreaterThan;
            let position_case_sensitive_regular_expression = crate::filters::PositionCaseSensitiveRegularExpression;
            let position_case_insensitive_regular_expression = crate::filters::PositionCaseInsensitiveRegularExpression;
            let contains_all_elements_of_array = crate::filters::ContainsAllElementsOfArray;
            // let contained_in_array = crate::filters::ContainedInArray;
            let overlaps_with_array = crate::filters::OverlapsWithArray;
            let all_elements_equal = crate::filters::AllElementsEqual;
            let contains_element_greater_than = crate::filters::ContainsElementGreaterThan;
            let all_elements_greater_than = crate::filters::AllElementsGreaterThan;
            let contains_element_case_sensitive_regular_expression = crate::filters::ContainsElementCaseSensitiveRegularExpression;
            let contains_element_case_insensitive_regular_expression = crate::filters::ContainsElementCaseInsensitiveRegularExpression;
            let all_elements_case_sensitive_regular_expression = crate::filters::AllElementsCaseSensitiveRegularExpression;
            let all_elements_case_insensitive_regular_expression = crate::filters::AllElementsCaseInsensitiveRegularExpression;

            let (
                maybe_postgresql_json_type_ident_where_element_position_equal,
                maybe_postgresql_json_type_ident_where_element_position_greater_than,
                maybe_postgresql_json_type_ident_where_element_position_case_sensitive_regular_expression,
                maybe_postgresql_json_type_ident_where_element_position_case_insensitive_regular_expression,
                maybe_postgresql_json_type_ident_where_element_contains_all_elements_of_array,
                // maybe_postgresql_json_type_ident_where_element_contained_in_array,
                maybe_postgresql_json_type_ident_where_element_overlaps_with_array,
                maybe_postgresql_json_type_ident_where_element_all_elements_equal,
                maybe_postgresql_json_type_ident_where_element_contains_element_greater_than,
                maybe_postgresql_json_type_ident_where_element_all_elements_greater_than,
                maybe_postgresql_json_type_ident_where_element_contains_element_case_sensitive_regular_expression,
                maybe_postgresql_json_type_ident_where_element_contains_element_case_insensitive_regular_expression,
                maybe_postgresql_json_type_ident_where_element_all_elements_case_sensitive_regular_expression,
                maybe_postgresql_json_type_ident_where_element_all_elements_case_insensitive_regular_expression,
            ) = match postgresql_json_type_variant.try_to_vec_element_type() {
                Ok(value) => (
                    //todo maybe should use value type in regular expression
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &position_equal,
                        token_stream: position_equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant, &value),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &position_greater_than,
                        token_stream: position_greater_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant, &value),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &position_case_sensitive_regular_expression,
                        token_stream: position_case_sensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &position_case_insensitive_regular_expression,
                        token_stream: position_case_insensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &contains_all_elements_of_array,
                        token_stream: contains_all_elements_of_array.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant, &value),
                    },
                    // MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                    //     where_operator_name: &contained_in_array,
                    //     token_stream: contained_in_array.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                    //         &postgresql_json_type_variant,
                    //         &value,
                    //     )
                    // },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &overlaps_with_array,
                        token_stream: overlaps_with_array.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant, &value),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &all_elements_equal,
                        token_stream: all_elements_equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&postgresql_json_type_ident_wrapper, &value),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &contains_element_greater_than,
                        token_stream: contains_element_greater_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant, &value),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &all_elements_greater_than,
                        token_stream: all_elements_greater_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant, &value),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &contains_element_case_sensitive_regular_expression,
                        token_stream: contains_element_case_sensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &contains_element_case_insensitive_regular_expression,
                        token_stream: contains_element_case_insensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &all_elements_case_sensitive_regular_expression,
                        token_stream: all_elements_case_sensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant),
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &all_elements_case_insensitive_regular_expression,
                        token_stream: all_elements_case_insensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant),
                    },
                ),
                Err(_) => (
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    // MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                ),
            };

            let mut common_postgresql_json_type_vec_filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = {
                let mut vec: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_filters_variants.clone();
                vec.push(&length_equal);
                vec.push(&length_more_than);
                vec
            };
            let mut common_postgresql_json_type_vec_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = {
                let mut vec: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_filters_token_stream.clone();
                vec.push(postgresql_json_type_ident_where_element_length_equal_token_stream);
                vec.push(postgresql_json_type_ident_where_element_length_more_than_token_stream);
                vec
            };

            if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_position_equal {
                common_postgresql_json_type_vec_filters_variants.push(where_operator_name);
                common_postgresql_json_type_vec_filters_token_stream.push(token_stream);
            }
            if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_contains_all_elements_of_array {
                common_postgresql_json_type_vec_filters_variants.push(where_operator_name);
                common_postgresql_json_type_vec_filters_token_stream.push(token_stream);
            }
            //will not use it coz do not understand how it works
            // if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_contained_in_array {
            //     common_postgresql_json_type_vec_filters_variants.push(where_operator_name);
            //     common_postgresql_json_type_vec_filters_token_stream.push(token_stream);
            // }
            if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_overlaps_with_array {
                common_postgresql_json_type_vec_filters_variants.push(where_operator_name);
                common_postgresql_json_type_vec_filters_token_stream.push(token_stream);
            }
            if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_all_elements_equal {
                common_postgresql_json_type_vec_filters_variants.push(where_operator_name);
                common_postgresql_json_type_vec_filters_token_stream.push(token_stream);
            }

            let generate_postgresql_json_type_where_element_number_token_stream = || {
                //todo maybe remove ident, field_type from arguments. variant is enough
                let greater_than = crate::filters::GreaterThan;
                let postgresql_json_type_ident_where_element_greater_than_token_stream = greater_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);
                let between = crate::filters::Between;
                let postgresql_json_type_ident_where_element_between_token_stream = between.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd, postgresql_json_type_variant);
                let in_handle = crate::filters::In;
                let postgresql_json_type_ident_where_element_in_token_stream = in_handle.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);
                //todo write wrapper around it with reuse parameters
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_where_element_token_stream(
                    &{
                        let mut vec = common_postgresql_json_type_filters_variants.clone();
                        vec.push(&greater_than);
                        vec.push(&between);
                        vec.push(&in_handle);
                        vec
                    },
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote! {
                    #(#common_postgresql_json_type_filters_token_stream)*

                    #postgresql_json_type_ident_where_element_greater_than_token_stream
                    #postgresql_json_type_ident_where_element_between_token_stream
                    #postgresql_json_type_ident_where_element_in_token_stream

                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_bool_token_stream = || {
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_where_element_token_stream(
                    &common_postgresql_json_type_filters_variants,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote! {
                    #(#common_postgresql_json_type_filters_token_stream)*

                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_string_token_stream = || {
                let case_sensitive_regular_expression = crate::filters::CaseSensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&ident);
                let case_insensitive_regular_expression = crate::filters::CaseInsensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&ident);

                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_where_element_token_stream(
                    &{
                        let mut vec = common_postgresql_json_type_filters_variants.clone();
                        vec.push(&case_sensitive_regular_expression);
                        vec.push(&case_insensitive_regular_expression);
                        vec
                    },
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote! {
                    #(#common_postgresql_json_type_filters_token_stream)*

                    #postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream
                    #postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream

                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };

            let generate_postgresql_json_type_where_element_vec_number_token_stream = || {
                let mut filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
                let mut filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_vec_filters_token_stream.clone();
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_position_greater_than {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_contains_element_greater_than {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_all_elements_greater_than {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                let postgresql_json_type_ident_where_element_token_stream =
                    generate_postgresql_type_where_element_token_stream(&filters_variants, &postgresql_json_type_ident_where_element_upper_camel_case, &postgresql_json_type_ident_where_element_upper_camel_case, &ShouldDeriveSchemarsJsonSchema::True);
                let generated = quote::quote! {
                    #(#filters_token_stream)*
                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_vec_bool_token_stream = || {
                let filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
                let filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_vec_filters_token_stream.clone();
                let postgresql_json_type_ident_where_element_token_stream =
                    generate_postgresql_type_where_element_token_stream(&filters_variants, &postgresql_json_type_ident_where_element_upper_camel_case, &postgresql_json_type_ident_where_element_upper_camel_case, &ShouldDeriveSchemarsJsonSchema::True);
                let generated = quote::quote! {
                    #(#filters_token_stream)*
                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_vec_string_token_stream = || {
                let mut filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
                let mut filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_vec_filters_token_stream.clone();
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_position_case_sensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_position_case_insensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_contains_element_case_sensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_contains_element_case_insensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_all_elements_case_sensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_all_elements_case_insensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                let postgresql_json_type_ident_where_element_token_stream =
                    generate_postgresql_type_where_element_token_stream(&filters_variants, &postgresql_json_type_ident_where_element_upper_camel_case, &postgresql_json_type_ident_where_element_upper_camel_case, &ShouldDeriveSchemarsJsonSchema::True);
                let generated = quote::quote! {
                    #(#filters_token_stream)*
                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            match &postgresql_json_type_variant.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                PostgresqlJsonTypePatternType::FullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => generate_postgresql_json_type_where_element_number_token_stream(),
                    PostgresqlJsonTypeSpecific::Bool => generate_postgresql_json_type_where_element_bool_token_stream(),
                    PostgresqlJsonTypeSpecific::String => generate_postgresql_json_type_where_element_string_token_stream(),
                },
                PostgresqlJsonTypePatternType::StdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => generate_postgresql_json_type_where_element_vec_number_token_stream(),
                    PostgresqlJsonTypeSpecific::Bool => generate_postgresql_json_type_where_element_vec_bool_token_stream(),
                    PostgresqlJsonTypeSpecific::String => generate_postgresql_json_type_where_element_vec_string_token_stream(),
                },
                PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => generate_postgresql_json_type_where_element_vec_number_token_stream(),
                    PostgresqlJsonTypeSpecific::Bool => generate_postgresql_json_type_where_element_vec_bool_token_stream(),
                    PostgresqlJsonTypeSpecific::String => generate_postgresql_json_type_where_element_vec_string_token_stream(),
                },
            }
        };
        //
        let postgresql_json_type_ident_where_element_second_dimension_token_stream = {
            let postgresql_json_type_ident_where_element_second_dimension_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfWhereElementSecondDimensionUpperCamelCase::from_tokens(&ident);

            let equal_second_dimension = crate::filters::EqualSecondDimension;
            let postgresql_json_type_ident_where_element_equal_second_dimension_token_stream = equal_second_dimension.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);

            let common_postgresql_json_type_filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = vec![&equal_second_dimension];
            let common_postgresql_json_type_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = vec![postgresql_json_type_ident_where_element_equal_second_dimension_token_stream];

            // let length_equal_second_dimension = crate::filters::LengthEqualSecondDimension;
            // let postgresql_json_type_ident_where_element_length_equal_token_stream = length_equal_second_dimension.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
            //     &postgresql_json_type_variant,
            // );

            let common_postgresql_json_type_vec_filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_filters_variants.clone();
            let common_postgresql_json_type_vec_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_filters_token_stream.clone();

            let generate_postgresql_json_type_where_element_vec_string_second_dimension_token_stream = || {
                let filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
                let filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_vec_filters_token_stream.clone();
                let postgresql_json_type_ident_where_element_second_dimension_token_stream = generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &postgresql_json_type_ident_where_element_second_dimension_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote! {
                    #(#filters_token_stream)*
                    #postgresql_json_type_ident_where_element_second_dimension_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            match &postgresql_json_type_variant.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                PostgresqlJsonTypePatternType::FullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::Bool => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::String => &proc_macro2_token_stream_new,
                },
                PostgresqlJsonTypePatternType::StdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::Bool => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::String => &generate_postgresql_json_type_where_element_vec_string_second_dimension_token_stream(),
                },
                PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::Bool => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::String => &proc_macro2_token_stream_new,
                },
            }
        };
        // println!("{}", quote::quote!{#postgresql_json_type_ident_where_element_second_dimension_token_stream});
        //
        let postgresql_json_type_ident_option_to_update_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfOptionToUpdateUpperCamelCase::from_tokens(&ident);
        let postgresql_json_type_ident_option_to_update_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_json_type_ident_option_to_update_upper_camel_case, &ident);
        let postgresql_json_type_ident_option_to_update_try_generate_postgresql_json_type_error_named_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamedUpperCamelCase::from_tokens(&ident);
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let postgresql_json_type_ident_option_to_update_try_generate_bind_increments_error_named_token_stream = {
            quote::quote! {
                #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #postgresql_json_type_ident_option_to_update_try_generate_postgresql_json_type_error_named_upper_camel_case {
                    #checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
                }
            }
        };
        let impl_crate_generate_postgresql_json_type_postgresql_json_type_for_ident_token_stream = postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
            &quote::quote! {crate::postgresql_json_type::postgresql_json_type_trait::},
            &ident,
            &postgresql_json_type_ident_to_create_upper_camel_case,
            &{
                //todo reuse
                let crate_json_types_postgresql_json_type_try_generate_postgresql_json_type_to_create_error_named_token_stream = quote::quote! {
                    crate::postgresql_json_type::postgresql_json_type_trait::TryGenerateToCreateErrorNamed
                };
                quote::quote! {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!("${increment}"))
                        }
                        None => Err(#crate_json_types_postgresql_json_type_try_generate_postgresql_json_type_to_create_error_named_token_stream::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!()
                        }),
                    }
                }
            },
            &{
                let self_to_create_snake_case = naming::SelfToCreateSnakeCase;
                quote::quote! {
                    query = query.bind(sqlx::types::Json(#self_to_create_snake_case.0));
                    query
                }
            },
            &postgresql_json_type_ident_field_reader_upper_camel_case,
            &postgresql_json_type_ident_options_to_read_upper_camel_case,
            &{
                let self_field_reader_snake_case = naming::SelfFieldReaderSnakeCase;
                let postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream = |format_handle_token_stream: &dyn quote::ToTokens| {
                    let pagination_start_end_initialization_token_stream = macros_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream(&self_field_reader_snake_case);
                    quote::quote! {
                        #pagination_start_end_initialization_token_stream
                        format!(#format_handle_token_stream)
                    }
                };
                let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;

                match &postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                    PostgresqlJsonTypePatternType::FullTypePath => {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('{{field_ident}}', jsonb_build_object('value', {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}'))"));
                        quote::quote! {
                            format!(#format_handle_token_stream)
                        }
                    }
                    PostgresqlJsonTypePatternType::StdVecVecFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_quotes::double_quotes_token_stream(&format!(
                        "jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))"
                    ))),
                    // PostgresqlJsonTypePatternType::StdVecVecStdOptionOptionFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                    //     &generate_quotes::double_quotes_token_stream(
                    //         &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value', case when jsonb_typeof({{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}') = 'array' then (select jsonb_agg(value) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}}) else null end))")
                    //     )
                    // ),
                    PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_quotes::double_quotes_token_stream(&format!(
                        "jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))"
                    ))),
                }
            },
            &postgresql_json_type_ident_where_element_upper_camel_case,
            &postgresql_json_type_ident_where_upper_camel_case,
            &postgresql_json_type_ident_option_to_update_upper_camel_case,
            &postgresql_json_type_ident_option_to_update_try_generate_postgresql_json_type_error_named_upper_camel_case,
            &{
                let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{jsonb_set_path}}}}}}',${{increment}})"));
                let self_option_to_update_try_generate_error_named_upper_camel_case = naming::SelfOptionToUpdateTryGenerateErrorNamedUpperCamelCase;
                quote::quote! {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!(#format_handle_token_stream))
                        }
                        None => Err(Self::#self_option_to_update_try_generate_error_named_upper_camel_case::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                    }
                }
            },
            &{
                let self_option_to_update_snake_case = naming::SelfOptionToUpdateSnakeCase;
                quote::quote! {
                    query = query.bind(sqlx::types::Json(#self_option_to_update_snake_case.0));
                    query
                }
            },
        );
        //todo maybe impl Encode instead of just wrap into sqlx::types::Json
        let generated = quote::quote! {
            #ident_token_stream
            #maybe_impl_schemars_json_schema_for_ident_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_token_stream

            #postgresql_json_type_ident_to_create_alias_token_stream
            #postgresql_json_type_ident_field_reader_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream
            #postgresql_json_type_ident_options_to_read_alias_token_stream
            #postgresql_json_type_where_token_stream
            #postgresql_json_type_ident_where_element_token_stream
            #postgresql_json_type_ident_where_element_second_dimension_token_stream
            #postgresql_json_type_ident_option_to_update_alias_token_stream
            #postgresql_json_type_ident_option_to_update_try_generate_bind_increments_error_named_token_stream
            #impl_crate_generate_postgresql_json_type_postgresql_json_type_for_ident_token_stream
        };
        // println!("{}", quote::quote!{#ident});
        // if quote::quote!{#ident}.to_string() == "StdVecVecStdVecVecUuidUuid" {
        //    //  println!("{generated}");
        //    //  println!("-------");
        //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeTime",
        //         &generated,
        //     );
        //     generated = quote::quote!{}
        // }
        generated
    }
    // PostgresqlJsonTypeVariant::all_variants()
    let variants_token_stream = [
        // PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: PostgresqlJsonTypeHandle::StdPrimitiveI8,
        //     postgresql_json_type_pattern: PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::False,
        //         postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::FullTypePath,
        //     },
        // },
        PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::False,
                postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::FullTypePath,
            },
        },
        PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::True,
                postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::FullTypePath,
            },
        },
        PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::False,
                postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::StdVecVecFullTypePath,
            },
        },
        // PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
        //     postgresql_json_type_pattern: PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::True,
        //         postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::StdVecVecFullTypePath,
        //     },
        // },
        // PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
        //     postgresql_json_type_pattern: PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::False,
        //         postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath,
        //     },
        // },
        // PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
        //     postgresql_json_type_pattern: PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::True,
        //         postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath,
        //     },
        // },
    ]
    .into_iter()
    .map(|element| generate_postgresql_json_type_handle_token_stream(&element));
    let generated = quote::quote! {
        #(#variants_token_stream)*
    };
    //  if ident == "" {
    //      println!("{generated}");
    //      println!("-------");
    //  }
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeTime",
    //     &generated,
    // );
    generated.into()
}

#[proc_macro]
pub fn generate_postgresql_types(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();

    let acc_snake_case = naming::AccSnakeCase;
    let element_snake_case = naming::ElementSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let error_snake_case = naming::ErrorSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let self_snake_case = naming::SelfSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let start_snake_case = naming::StartSnakeCase;
    let end_snake_case = naming::EndSnakeCase;
    let digits_snake_case = naming::DigitsSnakeCase;
    let scale_snake_case = naming::ScaleSnakeCase;
    let year_snake_case = naming::YearSnakeCase;
    let month_snake_case = naming::MonthSnakeCase;
    let day_snake_case = naming::DaySnakeCase;
    let months_snake_case = naming::MonthsSnakeCase;
    let days_snake_case = naming::DaysSnakeCase;
    let microseconds_snake_case = naming::MicrosecondsSnakeCase;

    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
    let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
    let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
    let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
    let std_primitive_u8_token_stream = token_patterns::StdPrimitiveU8;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = token_patterns::CrateGeneratePostgresqlJsonTypeStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;

    #[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    enum PostgresqlType {
        StdPrimitiveI16AsPostgresqlInt2,
        StdPrimitiveI32AsPostgresqlInt4,
        StdPrimitiveI64AsPostgresqlInt8,
        StdPrimitiveF32AsPostgresqlFloat4,
        StdPrimitiveF64AsPostgresqlFloat8,
        StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql,
        StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql,
        StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql,
        SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
        SqlxTypesDecimalAsPostgresqlNumeric,
        SqlxTypesBigDecimalAsPostgresqlNumeric,
        StdPrimitiveBoolAsPostgresqlBool,
        StdStringStringAsPostgresqlCharN,
        StdStringStringAsPostgresqlVarchar,
        StdStringStringAsPostgresqlText,
        StdVecVecStdPrimitiveU8AsPostgresqlBytea,
        SqlxTypesChronoNaiveTimeAsPostgresqlTime,
        SqlxTypesTimeTimeAsPostgresqlTime,
        SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
        SqlxTypesTimeDateAsPostgresqlDate,
        SqlxTypesChronoNaiveDateAsPostgresqlDate,
        SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
        SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
        SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
        SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql,
        SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient,
        SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
        SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
        SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
        SqlxTypesBitVecAsPostgresqlBit,
        SqlxTypesBitVecAsPostgresqlVarbit,
        SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
        SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
        SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
        SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
        SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange,
    }
    enum CanBeNullable {
        True,
        False,
    }
    impl PostgresqlType {
        fn can_be_nullable(&self) -> CanBeNullable {
            match &self {
                Self::StdPrimitiveI16AsPostgresqlInt2 => CanBeNullable::True,
                Self::StdPrimitiveI32AsPostgresqlInt4 => CanBeNullable::True,
                Self::StdPrimitiveI64AsPostgresqlInt8 => CanBeNullable::True,
                Self::StdPrimitiveF32AsPostgresqlFloat4 => CanBeNullable::True,
                Self::StdPrimitiveF64AsPostgresqlFloat8 => CanBeNullable::True,
                Self::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => CanBeNullable::False,
                Self::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => CanBeNullable::False,
                Self::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => CanBeNullable::False,
                Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => CanBeNullable::True,
                Self::SqlxTypesDecimalAsPostgresqlNumeric => CanBeNullable::True,
                Self::SqlxTypesBigDecimalAsPostgresqlNumeric => CanBeNullable::True,
                Self::StdPrimitiveBoolAsPostgresqlBool => CanBeNullable::True,
                Self::StdStringStringAsPostgresqlCharN => CanBeNullable::True,
                Self::StdStringStringAsPostgresqlVarchar => CanBeNullable::True,
                Self::StdStringStringAsPostgresqlText => CanBeNullable::True,
                Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime => CanBeNullable::True,
                Self::SqlxTypesTimeTimeAsPostgresqlTime => CanBeNullable::True,
                Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => CanBeNullable::True,
                Self::SqlxTypesTimeDateAsPostgresqlDate => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveDateAsPostgresqlDate => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => CanBeNullable::True,
                Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => CanBeNullable::True,
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => CanBeNullable::True,
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => CanBeNullable::True,
                Self::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => CanBeNullable::False,
                Self::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => CanBeNullable::True,
                Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => CanBeNullable::True,
                Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => CanBeNullable::True,
                Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => CanBeNullable::True,
                Self::SqlxTypesBitVecAsPostgresqlBit => CanBeNullable::True,
                Self::SqlxTypesBitVecAsPostgresqlVarbit => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => CanBeNullable::True,
            }
        }
        fn field_type_stringified(&self) -> std::string::String {
            let std_primitive_i16_stringified = "std::primitive::i16".to_string();
            let std_primitive_i32_stringified = "std::primitive::i32".to_string();
            let std_primitive_i64_stringified = "std::primitive::i64".to_string();
            let std_primitive_f32_stringified = "std::primitive::f32".to_string();
            let std_primitive_f64_stringified = "std::primitive::f64".to_string();
            let sqlx_postgres_types_pg_money_stringified = "sqlx::postgres::types::PgMoney".to_string();
            let sqlx_types_decimal_stringified = "sqlx::types::Decimal".to_string();
            let sqlx_types_big_decimal_stringified = "sqlx::types::BigDecimal".to_string();
            let std_primitive_bool_stringified = "std::primitive::bool".to_string();
            let std_string_string_stringified = "std::string::String".to_string();
            let std_vec_vec_std_primitive_u8_stringified = "std::vec::Vec<std::primitive::u8>".to_string();
            let sqlx_types_time_date_stringified = "sqlx::types::time::Date".to_string();
            let sqlx_types_chrono_naive_date_stringified = "sqlx::types::chrono::NaiveDate".to_string();
            let sqlx_types_chrono_naive_time_stringified = "sqlx::types::chrono::NaiveTime".to_string();
            let sqlx_types_time_time_stringified = "sqlx::types::time::Time".to_string();
            let sqlx_postgres_types_pg_interval_stringified = "sqlx::postgres::types::PgInterval".to_string();
            let sqlx_types_chrono_naive_date_time_stringified = "sqlx::types::chrono::NaiveDateTime".to_string();
            let sqlx_types_time_primitive_date_time_stringified = "sqlx::types::time::PrimitiveDateTime".to_string();
            let (sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified, sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified) = {
                let wrap_into_sqlx_types_chrono_date_time_stringified = |value: &dyn std::fmt::Display| format!("sqlx::types::chrono::DateTime<{value}>");
                (wrap_into_sqlx_types_chrono_date_time_stringified(&"sqlx::types::chrono::Utc"), wrap_into_sqlx_types_chrono_date_time_stringified(&"sqlx::types::chrono::Local"))
            };
            let sqlx_types_uuid_uuid_stringified = "sqlx::types::uuid::Uuid".to_string();
            let sqlx_types_ipnetwork_ip_network_stringified = "sqlx::types::ipnetwork::IpNetwork".to_string();
            let sqlx_types_mac_address_mac_address_stringified = "sqlx::types::mac_address::MacAddress".to_string();
            let sqlx_types_bit_vec_stringified = "sqlx::types::BitVec".to_string();
            let (
                sqlx_postgres_types_pg_range_std_primitive_i32_stringified,
                sqlx_postgres_types_pg_range_std_primitive_i64_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_time_date_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_decimal_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_big_decimal_stringified,
            ) = {
                let wrap_into_sqlx_postgres_types_pg_range_stringified = |value: &dyn std::fmt::Display| format!("sqlx::postgres::types::PgRange<{value}>");
                (
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&std_primitive_i32_stringified),
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&std_primitive_i64_stringified),
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_naive_date_time_stringified),
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_time_primitive_date_time_stringified),
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified),
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified),
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_naive_date_stringified),
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_time_date_stringified),
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_decimal_stringified),
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_big_decimal_stringified),
                )
            };
            match &self {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => std_primitive_i16_stringified,
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => std_primitive_i32_stringified,
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => std_primitive_i64_stringified,
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => std_primitive_f32_stringified,
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => std_primitive_f64_stringified,
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => std_primitive_i16_stringified,
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => std_primitive_i32_stringified,
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => std_primitive_i64_stringified,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => sqlx_postgres_types_pg_money_stringified,
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => sqlx_types_decimal_stringified,
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => sqlx_types_big_decimal_stringified,
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => std_primitive_bool_stringified,
                PostgresqlType::StdStringStringAsPostgresqlCharN => std_string_string_stringified,
                PostgresqlType::StdStringStringAsPostgresqlVarchar => std_string_string_stringified,
                PostgresqlType::StdStringStringAsPostgresqlText => std_string_string_stringified,
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => std_vec_vec_std_primitive_u8_stringified,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => sqlx_types_chrono_naive_time_stringified,
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => sqlx_types_time_time_stringified,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => sqlx_postgres_types_pg_interval_stringified,
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => sqlx_types_time_date_stringified,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => sqlx_types_chrono_naive_date_stringified,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => sqlx_types_chrono_naive_date_time_stringified,
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => sqlx_types_time_primitive_date_time_stringified,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => sqlx_types_uuid_uuid_stringified,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => sqlx_types_uuid_uuid_stringified,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => sqlx_types_ipnetwork_ip_network_stringified,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => sqlx_types_ipnetwork_ip_network_stringified,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => sqlx_types_mac_address_mac_address_stringified,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => sqlx_types_bit_vec_stringified,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => sqlx_types_bit_vec_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => sqlx_postgres_types_pg_range_std_primitive_i32_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => sqlx_postgres_types_pg_range_std_primitive_i64_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => sqlx_postgres_types_pg_range_sqlx_types_decimal_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => sqlx_postgres_types_pg_range_sqlx_types_big_decimal_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => sqlx_postgres_types_pg_range_sqlx_types_time_date_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified,
            }
        }
        fn field_type_token_stream(&self) -> proc_macro2::TokenStream {
            self.field_type_stringified().parse::<proc_macro2::TokenStream>().unwrap()
        }
    }
    impl quote::ToTokens for PostgresqlType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse PostgresqlType to proc_macro2::TokenStream")).to_tokens(tokens)
        }
    }
    impl std::convert::From<&RangeType> for PostgresqlType {
        fn from(value: &RangeType) -> PostgresqlType {
            match value {
                RangeType::StdPrimitiveI32AsPostgresqlInt4 => Self::StdPrimitiveI32AsPostgresqlInt4,
                RangeType::StdPrimitiveI64AsPostgresqlInt8 => Self::StdPrimitiveI64AsPostgresqlInt8,
                RangeType::SqlxTypesDecimalAsPostgresqlNumeric => Self::SqlxTypesDecimalAsPostgresqlNumeric,
                RangeType::SqlxTypesBigDecimalAsPostgresqlNumeric => Self::SqlxTypesBigDecimalAsPostgresqlNumeric,
                RangeType::SqlxTypesTimeDateAsPostgresqlDate => Self::SqlxTypesTimeDateAsPostgresqlDate,
                RangeType::SqlxTypesChronoNaiveDateAsPostgresqlDate => Self::SqlxTypesChronoNaiveDateAsPostgresqlDate,
                RangeType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
                RangeType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
                RangeType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
                RangeType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
            }
        }
    }
    enum RangeType {
        StdPrimitiveI32AsPostgresqlInt4,
        StdPrimitiveI64AsPostgresqlInt8,
        SqlxTypesDecimalAsPostgresqlNumeric,
        SqlxTypesBigDecimalAsPostgresqlNumeric,
        SqlxTypesTimeDateAsPostgresqlDate,
        SqlxTypesChronoNaiveDateAsPostgresqlDate,
        SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
        SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
        SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
    }
    // impl RangeType {
    //     fn field_type_stringified(&self) -> std::string::String {
    //         PostgresqlType::from(self).field_type_stringified()
    //     }
    //     fn field_type_token_stream(&self) -> proc_macro2::TokenStream {
    //         PostgresqlType::from(self).field_type_token_stream()
    //     }
    // }
    impl std::convert::TryFrom<PostgresqlType> for RangeType {
        type Error = ();
        fn try_from(value: PostgresqlType) -> Result<Self, Self::Error> {
            match &value {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => Err(()),
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => Err(()),
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => Err(()),
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => Err(()),
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => Err(()),
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => Err(()),
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => Err(()),
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => Err(()),
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => Err(()),
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => Err(()),
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => Err(()),
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => Err(()),
                PostgresqlType::StdStringStringAsPostgresqlCharN => Err(()),
                PostgresqlType::StdStringStringAsPostgresqlVarchar => Err(()),
                PostgresqlType::StdStringStringAsPostgresqlText => Err(()),
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => Err(()),
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => Err(()),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => Err(()),
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Err(()),
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Err(()),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => Err(()),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => Err(()),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => Err(()),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Err(()),
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Err(()),
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => Err(()),
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => Err(()),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => Ok(Self::StdPrimitiveI32AsPostgresqlInt4),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => Ok(Self::StdPrimitiveI64AsPostgresqlInt8),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => Ok(Self::SqlxTypesDecimalAsPostgresqlNumeric),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => Ok(Self::SqlxTypesBigDecimalAsPostgresqlNumeric),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => Ok(Self::SqlxTypesTimeDateAsPostgresqlDate),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => Ok(Self::SqlxTypesChronoNaiveDateAsPostgresqlDate),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => Ok(Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => Ok(Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz),
            }
        }
    }
    impl std::fmt::Display for RangeType {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "{}", naming::parameter::SelfNotNullUpperCamelCase::from_display(&PostgresqlType::from(self)))
        }
    }
    impl quote::ToTokens for RangeType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse RangeType to proc_macro2::TokenStream")).to_tokens(tokens)
        }
    }

    let generate_postgresql_type_token_stream = |postgresql_type: PostgresqlType| {
        let field_type = postgresql_type.field_type_token_stream();
        let generate_postgresql_type_not_null_or_nullable = |postgresql_type_not_null_or_nullable: &PostgresqlTypeNotNullOrNullable| -> proc_macro2::TokenStream {
            let postgresql_type_not_null_upper_camel_case = naming::parameter::SelfNotNullUpperCamelCase::from_tokens(&postgresql_type);
            let postgresql_type_not_null_or_nullable_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_not_null_or_nullable {
                PostgresqlTypeNotNullOrNullable::NotNull => &postgresql_type_not_null_upper_camel_case,
                PostgresqlTypeNotNullOrNullable::Nullable => &naming::parameter::SelfNullableUpperCamelCase::from_tokens(&postgresql_type),
            };
            let field_type_handle: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                PostgresqlTypeNotNullOrNullable::NotNull => &field_type,
                PostgresqlTypeNotNullOrNullable::Nullable => &quote::quote! {std::option::Option<#postgresql_type_not_null_upper_camel_case>},
            };
            let pub_struct_postgresql_type_not_null_or_nullable_token_stream = {
                let partial_ord_comma_token_stream = quote::quote! {PartialOrd,};
                let maybe_derive_partial_ord_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                    PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &partial_ord_comma_token_stream,
                        PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &partial_ord_comma_token_stream,
                        PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &partial_ord_comma_token_stream,
                        PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &partial_ord_comma_token_stream,
                        PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &partial_ord_comma_token_stream,
                        PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &partial_ord_comma_token_stream,
                        PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &partial_ord_comma_token_stream,
                        PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &partial_ord_comma_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &partial_ord_comma_token_stream,
                        PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &proc_macro2_token_stream_new,
                        PostgresqlType::StdStringStringAsPostgresqlCharN => &proc_macro2_token_stream_new,
                        PostgresqlType::StdStringStringAsPostgresqlVarchar => &proc_macro2_token_stream_new,
                        PostgresqlType::StdStringStringAsPostgresqlText => &proc_macro2_token_stream_new,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &partial_ord_comma_token_stream,
                        PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &partial_ord_comma_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &partial_ord_comma_token_stream,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &partial_ord_comma_token_stream,
                        PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &partial_ord_comma_token_stream,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                    },
                    PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                };
                let serde_serialize_comma_token_stream = quote::quote! {serde::Serialize,};
                let maybe_derive_serde_serialize_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                    PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdStringStringAsPostgresqlCharN => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdStringStringAsPostgresqlVarchar => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdStringStringAsPostgresqlText => &serde_serialize_comma_token_stream,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &serde_serialize_comma_token_stream,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                    },
                    PostgresqlTypeNotNullOrNullable::Nullable => &serde_serialize_comma_token_stream,
                };
                let serde_deserialize_comma_token_stream = quote::quote! {serde::Deserialize,};
                let maybe_derive_serde_deserialize_token_stream = match &postgresql_type_not_null_or_nullable {
                    PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdStringStringAsPostgresqlCharN => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdStringStringAsPostgresqlVarchar => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdStringStringAsPostgresqlText => &serde_deserialize_comma_token_stream,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &serde_deserialize_comma_token_stream,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                    },
                    PostgresqlTypeNotNullOrNullable::Nullable => &serde_deserialize_comma_token_stream,
                };
                quote::quote! {
                    #[derive(
                        Debug,
                        Clone,
                        PartialEq,
                        #maybe_derive_partial_ord_token_stream
                        #maybe_derive_serde_serialize_token_stream
                        #maybe_derive_serde_deserialize_token_stream
                    )]
                    pub struct #postgresql_type_not_null_or_nullable_upper_camel_case(pub #field_type_handle);
                }
            };

            let sqlx_types_time_primitive_date_time_as_postgresql_timestamp = PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp;
            let sqlx_types_time_date_as_postgresql_date = PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate;
            let sqlx_types_big_decimal_as_postgresql_numeric = PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric;

            let sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_field_type_token_stream = PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp.field_type_token_stream();
            let sqlx_types_time_primitive_date_time_as_postgresql_timestamp_field_type_token_stream = sqlx_types_time_primitive_date_time_as_postgresql_timestamp.field_type_token_stream();
            let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_field_type_token_stream = PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz.field_type_token_stream();
            let sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_field_type_token_stream = PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz.field_type_token_stream();
            let sqlx_types_chrono_naive_date_as_postgresql_date_field_type_token_stream = PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate.field_type_token_stream();
            let sqlx_types_time_date_as_postgresql_date_field_type_token_stream = sqlx_types_time_date_as_postgresql_date.field_type_token_stream();
            let sqlx_types_decimal_as_postgresql_numeric_field_type_token_stream = PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric.field_type_token_stream();
            let sqlx_types_big_decimal_as_postgresql_numeric_field_type_token_stream = sqlx_types_big_decimal_as_postgresql_numeric.field_type_token_stream();

            let sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null_upper_camel_case_token_stream = naming::parameter::SelfNotNullUpperCamelCase::from_display(&sqlx_types_time_primitive_date_time_as_postgresql_timestamp);
            let sqlx_types_time_date_as_postgresql_date_not_null_upper_camel_case_token_stream = naming::parameter::SelfNotNullUpperCamelCase::from_display(&sqlx_types_time_date_as_postgresql_date);
            let sqlx_types_big_decimal_as_postgresql_numeric_not_null_upper_camel_case_token_stream = naming::parameter::SelfNotNullUpperCamelCase::from_display(&sqlx_types_big_decimal_as_postgresql_numeric);

            let sqlx_postgres_types_pg_money_field_type_token_stream = PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney.field_type_token_stream();
            let sqlx_types_uuid_uuid_field_type_token_stream = PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient.field_type_token_stream();
            let sqlx_types_mac_address_mac_address_field_type_token_stream = PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr.field_type_token_stream();
            let sqlx_types_bit_vec_field_type_token_stream = PostgresqlType::SqlxTypesBitVecAsPostgresqlBit.field_type_token_stream();
            let sqlx_postgres_types_pg_interval_field_type_token_stream = PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval.field_type_token_stream();

            let std_vec_vec_std_primitive_bool_token_stream = quote::quote! {std::vec::Vec<std::primitive::bool>};
            let time_month_token_stream = quote::quote! {time::Month};
            let crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream = quote::quote! {crate::postgresql_type::postgresql_type::NumBigintBigInt};
            let sqlx_postgres_types_pg_range_token_stream = quote::quote! {sqlx::postgres::types::PgRange};
            let sqlx_types_time_time_midnight_token_stream = token_patterns::SqlxTypesTimeTimeMidnight;

            let generate_qlx_postgres_types_pg_range_start_end_token_stream = |start_token_stream: &dyn quote::ToTokens, end_token_stream: &dyn quote::ToTokens| {
                quote::quote! {#sqlx_postgres_types_pg_range_token_stream {
                    #start_snake_case: #start_token_stream,
                    #end_snake_case: #end_token_stream
                }}
            };

            let self_dot_zero_token_stream = quote::quote! {#self_snake_case.0};

            let impl_try_new_for_sqlx_types_time_date_token_stream = {
                let postgresql_type_not_null_try_new_error_named_upper_camel_case = naming::parameter::SelfNotNullTryNewErrorNamedUpperCamelCase::from_tokens(&postgresql_type);
                let from_calendar_date_upper_camel_case = naming::FromCalendarDateUpperCamelCase;
                let less_than_minimum_postgresql_value_upper_camel_case = naming::LessThanMinimumPostgresqlValueUpperCamelCase;
                let postgresql_type_not_null_try_new_error_named_token_stream = {
                    let error_content_token_stream = quote::quote! {
                        #[eo_to_std_string_string_serialize_deserialize]
                        value: #std_string_string_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    };
                    quote::quote! {
                        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #postgresql_type_not_null_try_new_error_named_upper_camel_case {
                            #from_calendar_date_upper_camel_case {
                                #error_content_token_stream
                            },
                            #less_than_minimum_postgresql_value_upper_camel_case {
                                #error_content_token_stream
                            },
                        }
                    }
                };
                let impl_postgresql_type_not_null_try_new_token_stream = {
                    let error_content_token_stream = quote::quote! {
                        value: format!("{value:?}"),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    };
                    quote::quote! {
                        impl #postgresql_type_not_null_upper_camel_case {
                            fn try_new(
                                #year_snake_case: std::primitive::i32,
                                #month_snake_case: #time_month_token_stream,
                                #day_snake_case: std::primitive::u8,
                            ) -> Result<Self, #postgresql_type_not_null_try_new_error_named_upper_camel_case> {
                                match #sqlx_types_time_date_as_postgresql_date_field_type_token_stream::from_calendar_date(
                                    #year_snake_case,
                                    #month_snake_case,
                                    #day_snake_case,
                                ) {
                                    Ok(value) => {
                                        //postgresql having minimum value "year": -4712, "month": 1, "day": 1. maximum "year": 5874897, "month": 12, "day": 31. but library type does not impl that correctly(in type max is 9999)
                                        let minimum = #sqlx_types_time_date_as_postgresql_date_field_type_token_stream::from_calendar_date(
                                            -4713,
                                            #time_month_token_stream::December,
                                            31,
                                        ).unwrap();
                                        if minimum > value {
                                            Err(#postgresql_type_not_null_try_new_error_named_upper_camel_case::#less_than_minimum_postgresql_value_upper_camel_case {
                                                #error_content_token_stream
                                            })
                                        }
                                        else {
                                            Ok(Self(value))
                                        }
                                    },
                                    Err(value) => Err(#postgresql_type_not_null_try_new_error_named_upper_camel_case::#from_calendar_date_upper_camel_case {
                                        #error_content_token_stream
                                    })
                                }
                            }
                        }
                    }
                };
                quote::quote! {
                    #postgresql_type_not_null_try_new_error_named_token_stream
                    #impl_postgresql_type_not_null_try_new_token_stream
                }
            };
            let maybe_impl_try_new_for_postgresql_type_not_null_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlText => &proc_macro2_token_stream_new,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &impl_try_new_for_sqlx_types_time_date_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                },
                PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
            };
            enum ParameterNumber {
                One,
                Two,
                Three,
            }
            impl ParameterNumber {
                fn get_index(&self) -> std::primitive::usize {
                    match &self {
                        Self::One => 0,
                        Self::Two => 1,
                        Self::Three => 2,
                    }
                }
                fn get_index_starting_with_one(&self) -> std::primitive::usize {
                    match &self {
                        Self::One => 1,
                        Self::Two => 2,
                        Self::Three => 3,
                    }
                }
                fn get_vec_from_index_starting_with_one(&self) -> std::vec::Vec<std::primitive::usize> {
                    (0..self.get_index_starting_with_one()).collect()
                }
            }
            let postgresql_type_not_null_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&postgresql_type_not_null_upper_camel_case);

            let generate_impl_serde_serialize_for_postgresql_type_not_null_tokens = |content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {
                    const _: () = {
                        #[allow(unused_extern_crates, clippy::useless_attribute)]
                        extern crate serde as _serde;
                        #[automatically_derived]
                        impl _serde::Serialize for #postgresql_type_not_null_upper_camel_case {
                            fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                #content_token_stream
                            }
                        }
                    };
                }
            };
            let generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4 = |value_token_stream: &dyn quote::ToTokens| {
                quote::quote! {_serde::Serializer::serialize_newtype_struct(__serializer, #postgresql_type_not_null_double_quotes_token_stream, &#self_dot_zero_token_stream #value_token_stream)}
            };
            let generate_serde_state_initialization_token_stream = |parameter_number: ParameterNumber| {
                let parameter_number_token_stream = {
                    let value = parameter_number.get_vec_from_index_starting_with_one().into_iter().map(|_| quote::quote! {+ 1});
                    quote::quote! {#(#value)*}
                };
                quote::quote! {
                    let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #postgresql_type_not_null_double_quotes_token_stream, false as std::primitive::usize #parameter_number_token_stream)?;
                }
            };
            let serde_state_initialization_two_fields_token_stream = generate_serde_state_initialization_token_stream(ParameterNumber::Two);
            let serde_state_initialization_three_fields_token_stream = generate_serde_state_initialization_token_stream(ParameterNumber::Three);
            let generate_serialize_field_token_stream = |field_name: &dyn std::fmt::Display, third_parameter_token_stream: &dyn quote::ToTokens| {
                let field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_name);
                quote::quote! {_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, #field_name_double_quotes_token_stream, #third_parameter_token_stream)?;}
            };
            let serde_ser_serialize_struct_end_token_stream = quote::quote! {_serde::ser::SerializeStruct::end(__serde_state)};
            let serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream = {
                let generate_self_zero_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {&#self_dot_zero_token_stream.#value_token_stream}
                };
                let start_serialize_field_token_stream = generate_serialize_field_token_stream(&start_snake_case, &generate_self_zero_tokens_token_stream(&start_snake_case));
                let end_serialize_field_token_stream = generate_serialize_field_token_stream(&end_snake_case, &generate_self_zero_tokens_token_stream(&end_snake_case));
                quote::quote! {
                    #serde_state_initialization_two_fields_token_stream
                    #start_serialize_field_token_stream
                    #end_serialize_field_token_stream
                    #serde_ser_serialize_struct_end_token_stream
                }
            };
            let generate_match_std_collections_bound_token_stream = |match_token_stream: &dyn quote::ToTokens, init_token_stream: &dyn quote::ToTokens| {
                quote::quote! {match #match_token_stream {
                    std::collections::Bound::Included(#value_snake_case) => std::collections::Bound::Included(#init_token_stream),
                    std::collections::Bound::Excluded(#value_snake_case) => std::collections::Bound::Excluded(#init_token_stream),
                    std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                }}
            };
            let generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25 = |type_token_stream: &dyn quote::ToTokens, is_need_to_be_cloned: std::primitive::bool| {
                let maybe_clone_token_stream: &dyn quote::ToTokens = if is_need_to_be_cloned { &quote::quote! {.clone()} } else { &proc_macro2_token_stream_new };
                let generate_self_zero_match_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens| {
                    let token_stream = generate_match_std_collections_bound_token_stream(&quote::quote! {#self_dot_zero_token_stream.#value_token_stream #maybe_clone_token_stream}, &quote::quote! {#type_token_stream(#value_snake_case)});
                    quote::quote! {&#token_stream}
                };
                let start_serialize_field_token_stream = generate_serialize_field_token_stream(&start_snake_case, &generate_self_zero_match_tokens_token_stream(&start_snake_case));
                let end_serialize_field_token_stream = generate_serialize_field_token_stream(&end_snake_case, &generate_self_zero_match_tokens_token_stream(&end_snake_case));
                quote::quote! {
                    #serde_state_initialization_two_fields_token_stream
                    #start_serialize_field_token_stream
                    #end_serialize_field_token_stream
                    #serde_ser_serialize_struct_end_token_stream
                }
            };
            let impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream = generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
            let impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream = generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.to_string()}));
            let impl_serde_serialize_for_sqlx_types_bit_vec_token_stream = generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&quote::quote! {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    #postgresql_type_not_null_double_quotes_token_stream,
                    &#self_dot_zero_token_stream.iter().collect::<#std_vec_vec_std_primitive_bool_token_stream>(),
                )
            });
            let maybe_impl_serde_serialize_for_postgresql_type_not_null_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.0})),
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&{
                        let digits_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::DigitsSnakeCase, &quote::quote! {&#crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream(bigint)});
                        let scale_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::ScaleSnakeCase, &quote::quote! {&exponent});
                        quote::quote! {
                            let (bigint, exponent) = #self_dot_zero_token_stream.clone().into_bigint_and_exponent();
                            #serde_state_initialization_two_fields_token_stream
                            #digits_serialize_field_token_stream
                            #scale_serialize_field_token_stream
                            #serde_ser_serialize_struct_end_token_stream
                        }
                    }),
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlText => &proc_macro2_token_stream_new,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&{
                        let generate_serialize_field_token_stream = |value: &dyn naming::StdFmtDisplayPlusQuoteToTokens| generate_serialize_field_token_stream(&value, &quote::quote! {&#self_dot_zero_token_stream.#value});
                        let months_serialize_field_token_stream = generate_serialize_field_token_stream(&months_snake_case);
                        let days_serialize_field_token_stream = generate_serialize_field_token_stream(&days_snake_case);
                        let microseconds_serialize_field_token_stream = generate_serialize_field_token_stream(&microseconds_snake_case);
                        quote::quote! {
                            #serde_state_initialization_three_fields_token_stream
                            #months_serialize_field_token_stream
                            #days_serialize_field_token_stream
                            #microseconds_serialize_field_token_stream
                            #serde_ser_serialize_struct_end_token_stream
                        }
                    }),
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&{
                        let generate_self_zero_tokens_token_stream = |value: &dyn naming::StdFmtDisplayPlusQuoteToTokens| generate_serialize_field_token_stream(&value, &quote::quote! {&#self_dot_zero_token_stream.#value()});
                        let year_serialize_field_token_stream = generate_self_zero_tokens_token_stream(&year_snake_case);
                        let month_serialize_field_token_stream = generate_self_zero_tokens_token_stream(&month_snake_case);
                        let day_serialize_field_token_stream = generate_self_zero_tokens_token_stream(&day_snake_case);
                        quote::quote! {
                            #serde_state_initialization_three_fields_token_stream
                            #year_serialize_field_token_stream
                            #month_serialize_field_token_stream
                            #day_serialize_field_token_stream
                            #serde_ser_serialize_struct_end_token_stream
                        }
                    }),
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.bytes()})),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &impl_serde_serialize_for_sqlx_types_bit_vec_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &impl_serde_serialize_for_sqlx_types_bit_vec_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => {
                        &generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(&sqlx_types_big_decimal_as_postgresql_numeric_not_null_upper_camel_case_token_stream, true))
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => {
                        &generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(&sqlx_types_time_date_as_postgresql_date_not_null_upper_camel_case_token_stream, false))
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => {
                        &generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(&sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null_upper_camel_case_token_stream, false))
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                },
                PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
            };

            let generate_std_collections_bound_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                quote::quote! {std::collections::Bound<#type_token_stream>}
            };

            //deserialize logic starting
            let struct_ident_double_quotes_token_stream = generate_struct_postgresql_type_where_element_tokens_double_quotes_token_stream(&postgresql_type);
            let postgresql_type_visitor_upper_camel_case = naming::parameter::SelfVisitorUpperCamelCase::from_tokens(&postgresql_type);

            let struct_visitor_token_stream = quote::quote! {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: serde::__private::PhantomData<#postgresql_type_not_null_upper_camel_case>,
                    lifetime: serde::__private::PhantomData<&'de ()>,
                }
            };

            let digits_scale_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 2] = [&digits_snake_case, &scale_snake_case];
            let year_month_day_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 3] = [&year_snake_case, &month_snake_case, &day_snake_case];
            let start_end_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 2] = [&start_snake_case, &end_snake_case];
            let months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 3] = [&months_snake_case, &days_snake_case, &microseconds_snake_case];

            let (serde_deserializer_deserialize_struct_visitor_token_stream, serde_deserializer_deserialize_struct_ident_visitor_token_stream) = {
                let generate_serde_deserializer_deserialize_struct_visitor_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #postgresql_type_not_null_double_quotes_token_stream,
                            FIELDS,
                            #content_token_stream
                        )
                    }
                };
                (
                    generate_serde_deserializer_deserialize_struct_visitor_token_stream(&quote::quote! {
                        __Visitor {
                            marker: _serde::__private::PhantomData::<#postgresql_type_not_null_upper_camel_case>,
                            lifetime: _serde::__private::PhantomData,
                        }
                    }),
                    generate_serde_deserializer_deserialize_struct_visitor_token_stream(&postgresql_type_visitor_upper_camel_case),
                )
            };

            let serde_deserializer_deserialize_newtype_struct_token_stream = quote::quote! {
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    #postgresql_type_not_null_double_quotes_token_stream,
                    __Visitor {
                        marker: serde::__private::PhantomData::<#postgresql_type_not_null_upper_camel_case>,
                        lifetime: serde::__private::PhantomData,
                    },
                )
            };

            let generate_impl_serde_deserialize_for_tokens_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {
                    const _: () = {
                        #[allow(unused_extern_crates, clippy::useless_attribute)]
                        extern crate serde as _serde;
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for #postgresql_type_not_null_upper_camel_case {
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                #content_token_stream
                            }
                        }
                    };
                }
            };

            let parameter_number_two = ParameterNumber::Two;
            let parameter_number_three = ParameterNumber::Three;

            let generate_field_index_token_stream = |index: std::primitive::usize| format!("__{}{index}", naming::FieldSnakeCase).parse::<proc_macro2::TokenStream>().unwrap();

            let (enum_field_two_token_stream, enum_field_three_token_stream) = {
                let generate_enum_field_token_stream = |parameter_number: &ParameterNumber| {
                    let fields_token_stream = {
                        let fields_token_stream = parameter_number.get_vec_from_index_starting_with_one().into_iter().map(&generate_field_index_token_stream);
                        quote::quote! {#(#fields_token_stream),*}
                    };
                    quote::quote! {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            #fields_token_stream,
                            __ignore,
                        }
                    }
                };
                (generate_enum_field_token_stream(&parameter_number_two), generate_enum_field_token_stream(&parameter_number_three))
            };

            let (fn_expecting_struct_ident_double_quotes_token_stream, fn_expecting_field_identifier_token_stream, fn_expecting_months_or_days_or_microseconds_token_stream, fn_expecting_start_or_end_token_stream) = {
                let generate_fn_expecting_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(__formatter, #content_token_stream)
                        }
                    }
                };
                (
                    generate_fn_expecting_token_stream(&struct_ident_double_quotes_token_stream),
                    generate_fn_expecting_token_stream(&quote::quote! {"field identifier"}),
                    generate_fn_expecting_token_stream(&quote::quote! {"`months` or `days` or `microseconds`"}),
                    generate_fn_expecting_token_stream(&quote::quote! {"`start` or `end`"}),
                )
            };

            let field_0_token_stream = generate_field_index_token_stream(ParameterNumber::One.get_index());
            let field_1_token_stream = generate_field_index_token_stream(ParameterNumber::Two.get_index());
            let field_2_token_stream = generate_field_index_token_stream(ParameterNumber::Three.get_index());

            let generate_serde_private_ok_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {serde::__private::Ok(#content_token_stream)}
            };
            let generate_serde_private_ok_postgresql_type_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_serde_private_ok_token_stream(&quote::quote! {#postgresql_type_not_null_upper_camel_case(#content_token_stream)});

            let match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream = quote::quote! {match #sqlx_types_uuid_uuid_field_type_token_stream::try_parse(&#field_0_token_stream) {
                Ok(value) => value,
                Err(error) => {
                    return Err(serde::de::Error::custom(error));
                }
            }};
            let sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream = quote::quote! {#sqlx_types_mac_address_mac_address_field_type_token_stream::new(#field_0_token_stream)};
            let array_std_primitive_u8_6_token_stream = quote::quote! {[std::primitive::u8; 6]};
            let sqlx_types_bit_vec_field_type_set_token_stream = quote::quote! {{
                let mut bit_vec = #sqlx_types_bit_vec_field_type_token_stream::from_elem(#field_0_token_stream.len(), false);
                #field_0_token_stream.into_iter().enumerate().for_each(|(index, element)|{
                    bit_vec.set(index, element);
                });
                bit_vec
            }};
            let generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream = |months_token_stream: &dyn quote::ToTokens, days_token_stream: &dyn quote::ToTokens, microseconds_token_stream: &dyn quote::ToTokens| {
                quote::quote! {#sqlx_postgres_types_pg_interval_field_type_token_stream {
                    #months_snake_case #months_token_stream,
                    #days_snake_case #days_token_stream,
                    #microseconds_snake_case #microseconds_token_stream
                }}
            };

            let (fn_visit_newtype_struct_pg_money_token_stream, fn_visit_newtype_struct_uuid_token_stream, fn_visit_newtype_struct_mac_address_token_stream, fn_visit_newtype_struct_bit_vec_token_stream) = {
                let generate_fn_visit_newtype_struct_token_stream = |type_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                    let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(content_token_stream);
                    quote::quote! {
                        #[inline]
                        fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
                        where
                            __E: serde::Deserializer<'de>,
                        {
                            let #field_0_token_stream = <#type_token_stream as serde::Deserialize>::deserialize(__e)?;
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    }
                };
                (
                    generate_fn_visit_newtype_struct_token_stream(&std_primitive_i64_token_stream, &quote::quote! {#sqlx_postgres_types_pg_money_field_type_token_stream(#field_0_token_stream)}),
                    generate_fn_visit_newtype_struct_token_stream(&std_string_string_token_stream, &match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream),
                    generate_fn_visit_newtype_struct_token_stream(&array_std_primitive_u8_6_token_stream, &sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream),
                    generate_fn_visit_newtype_struct_token_stream(&std_vec_vec_std_primitive_bool_token_stream, &sqlx_types_bit_vec_field_type_set_token_stream),
                )
            };

            let generate_fn_visit_seq_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: serde::de::SeqAccess<'de>,
                    {
                        #content_token_stream
                    }
                }
            };

            let generate_fields_serde_de_seq_access_next_element_initialization_token_stream = |vec_token_stream: &[&dyn quote::ToTokens]| {
                let error_message_token_stream = generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(&postgresql_type, vec_token_stream.len());
                let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                    let field_index_token_stream = generate_field_index_token_stream(index);
                    let index_usize_token_stream = format!("{index}usize").parse::<proc_macro2::TokenStream>().unwrap();
                    quote::quote! {
                        let #field_index_token_stream = match serde::de::SeqAccess::next_element::<#element>(&mut __seq)? {
                            serde::__private::Some(__value) => __value,
                            serde::__private::None => {
                                return serde::__private::Err(serde::de::Error::invalid_length(#index_usize_token_stream, &#error_message_token_stream));
                            }
                        };
                    }
                });
                quote::quote! {#(#fields_initialization_token_stream)*}
            };

            let serde_private_ok_postgresql_type_sqlx_types_big_decimal_new_field0_field1_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote! {#sqlx_types_big_decimal_as_postgresql_numeric_field_type_token_stream::new(
                #field_0_token_stream.0,
                #field_1_token_stream
            )});

            let fn_visit_seq_pg_money_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_primitive_i64_token_stream]);
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote! {#sqlx_postgres_types_pg_money_field_type_token_stream(#field_0_token_stream)});
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_types_big_decimal_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream, &std_primitive_i64_token_stream]);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_sqlx_types_big_decimal_new_field0_field1_token_stream
                }
            });
            let fn_visit_seq_sqlx_types_time_date_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_primitive_i32_token_stream, &time_month_token_stream, &std_primitive_u8_token_stream]);
                quote::quote! {
                    #fields_initialization_token_stream
                    match #postgresql_type_not_null_upper_camel_case::try_new(#field_0_token_stream, #field_1_token_stream, #field_2_token_stream) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                    }
                }
            });

            let (seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream, seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream, seq_next_element_ok_or_else_serde_de_error_invalid_length_two_token_stream) = {
                let generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream = |parameter_number: &ParameterNumber| {
                    let index_token_stream = match &parameter_number {
                        ParameterNumber::One => quote::quote! {0},
                        ParameterNumber::Two => quote::quote! {1},
                        ParameterNumber::Three => quote::quote! {2},
                    };
                    quote::quote! {__seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(#index_token_stream, &self))?;}
                };
                (
                    generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream(&ParameterNumber::One),
                    generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream(&ParameterNumber::Two),
                    generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream(&ParameterNumber::Three),
                )
            };
            let fn_visit_seq_sqlx_postgres_types_pg_interval_token_stream = generate_fn_visit_seq_token_stream(&{
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(&proc_macro2_token_stream_new, &proc_macro2_token_stream_new, &proc_macro2_token_stream_new));
                quote::quote! {
                    let #months_snake_case = #seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream
                    let #days_snake_case = #seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream
                    let #microseconds_snake_case = #seq_next_element_ok_or_else_serde_de_error_invalid_length_two_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let sqlx_postgres_types_pg_range_start_end_token_stream = generate_qlx_postgres_types_pg_range_start_end_token_stream(&field_0_token_stream, &field_1_token_stream);
            let sqlx_postgres_types_pg_range_bound_start_end_token_stream = {
                let value_zero_token_stream = quote::quote! {#value_snake_case.0};
                generate_qlx_postgres_types_pg_range_start_end_token_stream(
                    &generate_match_std_collections_bound_token_stream(&field_0_token_stream, &value_zero_token_stream),
                    &generate_match_std_collections_bound_token_stream(&field_1_token_stream, &value_zero_token_stream),
                )
            };
            let fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream = generate_fn_visit_seq_token_stream(&{
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote! {
                    let #field_0_token_stream = #seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream
                    let #field_1_token_stream = #seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_field_type_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null_upper_camel_case_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                        &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_field_type_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_field_type_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_chrono_naive_date_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_as_postgresql_date_field_type_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_time_date_as_postgresql_date_not_null_upper_camel_case_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream, &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let token_stream = generate_std_collections_bound_token_stream(&sqlx_types_decimal_as_postgresql_numeric_field_type_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&token_stream, &token_stream])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let token_stream = generate_std_collections_bound_token_stream(&sqlx_types_big_decimal_as_postgresql_numeric_not_null_upper_camel_case_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&token_stream, &token_stream])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_types_uuid_uuid_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_string_string_token_stream]);
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_types_mac_address_mac_address_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&array_std_primitive_u8_6_token_stream]);
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_types_bit_vec_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_vec_vec_std_primitive_bool_token_stream]);
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_types_bit_vec_field_type_set_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });

            let (fn_visit_u64_two_token_stream, fn_visit_u64_three_token_stream) = {
                let generate_fn_visit_u64_token_stream = |parameter_number: &ParameterNumber| {
                    let fields_token_stream = {
                        parameter_number.get_vec_from_index_starting_with_one().into_iter().map(|element| {
                            let index_variant_token_stream = format!("{element}u64").parse::<proc_macro2::TokenStream>().unwrap();
                            let field_index_token_stream = generate_field_index_token_stream(element);
                            quote::quote! {#index_variant_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)}
                        })
                    };
                    quote::quote! {
                        fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
                        where
                            __E: serde::de::Error,
                        {
                            match __value {
                                #(#fields_token_stream),*,
                                _ => serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                };
                (generate_fn_visit_u64_token_stream(&ParameterNumber::Two), generate_fn_visit_u64_token_stream(&ParameterNumber::Three))
            };
            let (fn_visit_str_value_digits_scale_token_stream, fn_visit_str_value_year_month_day_token_stream, fn_visit_str_value_start_end_token_stream) = {
                let generate_fn_visit_str_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                    let fields_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                        let element_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element);
                        let field_index_name_token_stream = generate_field_index_token_stream(index);
                        quote::quote! {
                            #element_double_quotes_token_stream => _serde::__private::Ok(__Field::#field_index_name_token_stream)
                        }
                    });
                    quote::quote! {
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                #(#fields_token_stream),*,
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                };
                (
                    generate_fn_visit_str_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_str_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_str_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                )
            };

            let (fn_visit_str_field_months_days_microseconds_token_stream, fn_visit_str_field_start_end_token_stream) = {
                let generate_fn_visit_str_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                    let fields_token_stream = vec_token_stream.iter().map(|element| {
                        let element_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element);
                        let element_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element);
                        quote::quote! {#element_double_quotes_token_stream => Ok(Field::#element_upper_camel_case_token_stream)}
                    });
                    quote::quote! {
                        fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: serde::de::Error,
                        {
                            match value {
                                #(#fields_token_stream),*,
                                _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
                };
                (
                    generate_fn_visit_str_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_str_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                )
            };

            let (fn_visit_bytes_digits_scale_token_stream, fn_visit_bytes_year_month_day_token_stream, fn_visit_bytes_start_end_token_stream) = {
                let generate_fn_visit_bytes_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                    let fields_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                        let b_element_double_quotes_token_stream = format!("b{}", generate_quotes::double_quotes_stringified(&element)).parse::<proc_macro2::TokenStream>().unwrap();
                        let field_index_name_token_stream = generate_field_index_token_stream(index);
                        quote::quote! {
                            #b_element_double_quotes_token_stream => serde::__private::Ok(__Field::#field_index_name_token_stream)
                        }
                    });
                    quote::quote! {
                        fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
                        where
                            __E: serde::de::Error,
                        {
                            match __value {
                                #(#fields_token_stream),*,
                                _ => serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                };
                (
                    generate_fn_visit_bytes_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_bytes_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_bytes_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                )
            };

            let serde_deserializer_deserialize_identifier_token_stream = quote::quote! {
                _serde::Deserializer::deserialize_identifier(
                    __deserializer,
                    __FieldVisitor,
                )
            };

            let impl_serde_deserialize_for_field_token_stream = quote::quote! {
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #serde_deserializer_deserialize_identifier_token_stream
                    }
                }
            };

            let (
                std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream,
                std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                std_collections_bound_sqlx_types_chrono_naive_date_token_stream,
                std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream,
                std_collections_bound_sqlx_types_decimal_token_stream,
                std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream,
            ) = {
                (
                    generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_field_type_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null_upper_camel_case_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_field_type_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_field_type_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_as_postgresql_date_field_type_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_time_date_as_postgresql_date_not_null_upper_camel_case_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_decimal_as_postgresql_numeric_field_type_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_big_decimal_as_postgresql_numeric_not_null_upper_camel_case_token_stream),
                )
            };

            let (
                fn_visit_map_sqlx_types_big_decimal_token_stream,
                fn_visit_map_sqlx_types_time_date_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
            ) = {
                let generate_fn_visit_map_token_stream =
                    |field_option_none_initialization_token_stream: &dyn quote::ToTokens, while_some_next_key_field_token_stream: &dyn quote::ToTokens, match_field_initialization_token_stream: &dyn quote::ToTokens, serde_private_ok_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            #[inline]
                            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: serde::de::MapAccess<'de>,
                            {
                                #field_option_none_initialization_token_stream
                                #while_some_next_key_field_token_stream
                                #match_field_initialization_token_stream
                                #serde_private_ok_token_stream
                            }
                        }
                    };

                let (
                    field_option_none_initialization_sqlx_types_big_decimal_token_stream,
                    field_option_none_initialization_sqlx_types_time_date_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                ) = {
                    let generate_field_option_none_initialization_token_stream = |vec_token_stream: &[&dyn quote::ToTokens]| {
                        let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                            let field_index_name_token_stream = generate_field_index_token_stream(index);
                            quote::quote! {
                                let mut #field_index_name_token_stream: serde::__private::Option<#element> = serde::__private::None;
                            }
                        });
                        quote::quote! {#(#fields_initialization_token_stream)*}
                    };
                    (
                        generate_field_option_none_initialization_token_stream(&[&crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream, &std_primitive_i64_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_primitive_i32_token_stream, &time_month_token_stream, &std_primitive_u8_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                            &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream, &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_decimal_token_stream, &std_collections_bound_sqlx_types_decimal_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream, &std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream]),
                    )
                };

                let (
                    while_some_next_key_field_sqlx_types_big_decimal_token_stream,
                    while_some_next_key_field_sqlx_types_time_date_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                ) = {
                    let generate_while_some_next_key_field_token_stream = |vec_token_stream: &[(&dyn std::fmt::Display, &dyn quote::ToTokens)]| {
                        let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                            let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&element.0);
                            let field_type_token_stream = &element.1;
                            let field_index_name_token_stream = generate_field_index_token_stream(index);
                            quote::quote! {
                                __Field::#field_index_name_token_stream => {
                                    if serde::__private::Option::is_some(&#field_index_name_token_stream) {
                                        return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field(#field_name_double_quotes_token_stream));
                                    }
                                    #field_index_name_token_stream = serde::__private::Some(serde::de::MapAccess::next_value::<#field_type_token_stream>(&mut __map)?);
                                }
                            }
                        });
                        quote::quote! {
                            while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                match __key {
                                    #(#fields_initialization_token_stream)*
                                    _ => {
                                        let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                                    }
                                }
                            }
                        }
                    };
                    (
                        generate_while_some_next_key_field_token_stream(&[(&digits_snake_case, &crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream), (&scale_snake_case, &std_primitive_i64_token_stream)]),
                        generate_while_some_next_key_field_token_stream(&[(&year_snake_case, &std_primitive_i32_token_stream), (&month_snake_case, &time_month_token_stream), (&day_snake_case, &std_primitive_u8_token_stream)]),
                        generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream), (&end_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream)]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream),
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream),
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream),
                        ]),
                        generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream), (&end_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream)]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream),
                        ]),
                        generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_collections_bound_sqlx_types_decimal_token_stream), (&end_snake_case, &std_collections_bound_sqlx_types_decimal_token_stream)]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream),
                        ]),
                    )
                };

                let (match_field_initialization_sqlx_types_big_decimal_token_stream, match_field_initialization_sqlx_types_time_date_token_stream, match_field_initialization_start_end_token_stream) = {
                    let generate_match_field_initialization_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                        let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                            let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&element);
                            let field_index_name_token_stream = generate_field_index_token_stream(index);
                            quote::quote! {
                                let #field_index_name_token_stream = match #field_index_name_token_stream {
                                    serde::__private::Some(#field_index_name_token_stream) => #field_index_name_token_stream,
                                    serde::__private::None => serde::__private::de::missing_field(#field_name_double_quotes_token_stream)?,
                                };
                            }
                        });
                        quote::quote! {#(#fields_initialization_token_stream)*}
                    };
                    (
                        generate_match_field_initialization_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                        generate_match_field_initialization_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                        generate_match_field_initialization_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                    )
                };

                let match_postgresql_type_try_new_field0_field1_field2_token_stream = quote::quote! {
                    match #postgresql_type_not_null_upper_camel_case::try_new(#field_0_token_stream, #field_1_token_stream, #field_2_token_stream) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                    }
                };
                let serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                let serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                (
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_types_big_decimal_token_stream,
                        &while_some_next_key_field_sqlx_types_big_decimal_token_stream,
                        &match_field_initialization_sqlx_types_big_decimal_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_types_big_decimal_new_field0_field1_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_types_time_date_token_stream,
                        &while_some_next_key_field_sqlx_types_time_date_token_stream,
                        &match_field_initialization_sqlx_types_time_date_token_stream,
                        &match_postgresql_type_try_new_field0_field1_field2_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream,
                    ),
                )
            };

            let generate_double_dot_space_tokens_token_stream = |value: &dyn quote::ToTokens| {
                quote::quote! {: #value}
            };

            let (fn_visit_map_sqlx_postgres_types_pg_interval_token_stream, fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream) = {
                let generate_fn_visit_map_token_stream =
                    |field_option_none_initialization_token_stream: &dyn quote::ToTokens, while_some_next_key_field_token_stream: &dyn quote::ToTokens, match_field_initialization_token_stream: &dyn quote::ToTokens, serde_private_ok_token_stream: &dyn quote::ToTokens| {
                        let serde_private_ok_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&serde_private_ok_token_stream);
                        quote::quote! {
                            #[inline]
                            fn visit_map<V>(self, mut map: V) -> Result<#postgresql_type_not_null_upper_camel_case, V::Error>
                            where
                                V: serde::de::MapAccess<'de>,
                            {
                                #field_option_none_initialization_token_stream
                                #while_some_next_key_field_token_stream
                                #match_field_initialization_token_stream
                                #serde_private_ok_token_stream
                            }
                        }
                    };

                let (field_option_none_initialization_months_days_microseconds_token_stream, field_option_none_initialization_start_end_token_stream) = {
                    let generate_field_option_none_initialization_token_stream = |vec_token_stream: &[&dyn quote::ToTokens]| {
                        let fields_initialization_token_stream = vec_token_stream.iter().map(|element| {
                            quote::quote! {
                                let mut #element = None;
                            }
                        });
                        quote::quote! {#(#fields_initialization_token_stream)*}
                    };
                    (
                        generate_field_option_none_initialization_token_stream(&[&months_snake_case, &days_snake_case, &microseconds_snake_case]),
                        generate_field_option_none_initialization_token_stream(&[&start_snake_case, &end_snake_case]),
                    )
                };

                let (while_some_next_key_field_months_days_microseconds_token_stream, while_some_next_key_field_start_end_token_stream) = {
                    let generate_while_some_next_key_field_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                        let fields_initialization_token_stream = vec_token_stream.iter().map(|element| {
                            let field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element);
                            let element_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element);
                            quote::quote! {
                                Field::#element_upper_camel_case_token_stream => {
                                    if #element.is_some() {
                                        return Err(serde::de::Error::duplicate_field(#field_name_double_quotes_token_stream));
                                    }
                                    #element = Some(map.next_value()?);
                                }
                            }
                        });
                        quote::quote! {
                            while let Some(key) = map.next_key()? {
                                match key {
                                    #(#fields_initialization_token_stream)*
                                }
                            }
                        }
                    };
                    (
                        generate_while_some_next_key_field_token_stream(&[&months_snake_case, &days_snake_case, &microseconds_snake_case]),
                        generate_while_some_next_key_field_token_stream(&[&start_snake_case, &end_snake_case]),
                    )
                };

                let (match_field_initialization_months_days_microseconds_token_stream, match_field_initialization_start_end_token_stream) = {
                    let generate_match_field_initialization_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                        let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                            let field_index_name_token_stream = generate_field_index_token_stream(index);
                            let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&element);
                            quote::quote! {
                                let #field_index_name_token_stream = #element.ok_or_else(|| serde::de::Error::missing_field(#field_name_double_quotes_token_stream))?;
                            }
                        });
                        quote::quote! {#(#fields_initialization_token_stream)*}
                    };
                    (
                        generate_match_field_initialization_token_stream(&[&months_snake_case, &days_snake_case, &microseconds_snake_case]),
                        generate_match_field_initialization_token_stream(&[&start_snake_case, &end_snake_case]),
                    )
                };
                (
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_months_days_microseconds_token_stream,
                        &while_some_next_key_field_months_days_microseconds_token_stream,
                        &match_field_initialization_months_days_microseconds_token_stream,
                        &generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(
                            &generate_double_dot_space_tokens_token_stream(&field_0_token_stream),
                            &generate_double_dot_space_tokens_token_stream(&field_1_token_stream),
                            &generate_double_dot_space_tokens_token_stream(&field_2_token_stream),
                        ),
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_start_end_token_stream,
                        &while_some_next_key_field_start_end_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                )
            };

            let (field_months_days_microseconds_token_stream, field_start_end_token_stream) = {
                let generate_field_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                    let variants_token_stream = vec_token_stream.iter().map(|element| naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element));
                    quote::quote! {
                        enum Field {
                            #(#variants_token_stream),*
                        }
                    }
                };
                (
                    generate_field_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                    generate_field_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                )
            };

            let (const_fields_sqlx_types_big_decimal_token_stream, const_fields_sqlx_types_time_date_token_stream, const_fields_sqlx_postgres_types_pg_interval_token_stream, const_fields_start_end_token_stream) = {
                let generate_const_fields_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                    let field_names_token_stream = vec_token_stream.iter().map(|element| generate_quotes::double_quotes_token_stream(&element));
                    quote::quote! {
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[#(#field_names_token_stream),*];
                    }
                };
                (
                    generate_const_fields_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                    generate_const_fields_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                    generate_const_fields_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                    generate_const_fields_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                )
            };

            let (
                impl_serde_de_visitor_for_visitor_pg_money_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_types_big_decimal_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_types_time_date_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                impl_serde_de_visitor_for_visitor_uuid_uuid_token_stream,
                impl_serde_de_visitor_for_visitor_mac_address_mac_address_token_stream,
                impl_serde_de_visitor_for_visitor_bit_vec_token_stream,
            ) = {
                let generate_impl_serde_de_visitor_for_visitor_token_stream = |first_token_stream: &dyn quote::ToTokens, second_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #postgresql_type_not_null_upper_camel_case;
                            #fn_expecting_struct_ident_double_quotes_token_stream
                            #first_token_stream
                            #second_token_stream
                        }
                    }
                };
                (
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_pg_money_token_stream, &fn_visit_seq_pg_money_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_types_big_decimal_token_stream, &fn_visit_map_sqlx_types_big_decimal_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_types_time_date_token_stream, &fn_visit_map_sqlx_types_time_date_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(
                        &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                        &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                    ),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(
                        &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                    ),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(
                        &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                    ),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_uuid_token_stream, &fn_visit_seq_sqlx_types_uuid_uuid_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_mac_address_token_stream, &fn_visit_seq_sqlx_types_mac_address_mac_address_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_bit_vec_token_stream, &fn_visit_seq_sqlx_types_bit_vec_token_stream),
                )
            };

            let generate_impl_serde_de_visitor_for_tokens_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {
                    impl<'de> _serde::de::Visitor<'de> for #ident_token_stream {
                        #content_token_stream
                    }
                }
            };

            let (
                impl_serde_de_visitor_for_field_visitor_token_stream_8ae918a4_5464_4f56_8078_ab475f269079,
                impl_serde_de_visitor_for_field_visitor_token_stream_77c8b6d8_4ac3_4551_8498_36b9d77317f2,
                impl_serde_de_visitor_for_field_visitor_token_stream_31609291_37e6_427f_8d04_d19e2af929f8,
                impl_serde_de_visitor_for_field_visitor_token_stream_ca843915_2330_4969_8bc8_8b33bff7a565,
                impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6,
            ) = {
                let generate_impl_serde_de_visitor_for_field_visitor_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                    let field_visitor_token_stream = quote::quote! {__FieldVisitor};
                    let impl_serde_de_visitor_for_tokens_token_stream = generate_impl_serde_de_visitor_for_tokens_token_stream(&field_visitor_token_stream, &content_token_stream);
                    quote::quote! {
                        #[doc(hidden)]
                        struct #field_visitor_token_stream;
                        #impl_serde_de_visitor_for_tokens_token_stream
                    }
                };
                (
                    generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_value_digits_scale_token_stream
                        #fn_visit_bytes_digits_scale_token_stream
                    }),
                    generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_three_token_stream
                        #fn_visit_str_value_year_month_day_token_stream
                        #fn_visit_bytes_year_month_day_token_stream
                    }),
                    generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                        type Value = Field;
                        #fn_expecting_months_or_days_or_microseconds_token_stream
                        #fn_visit_str_field_months_days_microseconds_token_stream
                    }),
                    generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                        type Value = Field;
                        #fn_expecting_start_or_end_token_stream
                        #fn_visit_str_field_start_end_token_stream
                    }),
                    generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_value_start_end_token_stream
                        #fn_visit_bytes_start_end_token_stream
                    }),
                )
            };

            let (impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_interval_token_stream, impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream) = {
                let generate_impl_serde_de_visitor_for_ident_visitor_token_stream = |first_token_stream: &dyn quote::ToTokens, second_token_stream: &dyn quote::ToTokens| {
                    let impl_serde_de_visitor_for_tokens_token_stream = generate_impl_serde_de_visitor_for_tokens_token_stream(
                        &postgresql_type_visitor_upper_camel_case,
                        &quote::quote! {
                            type Value = #postgresql_type_not_null_upper_camel_case;
                            #fn_expecting_struct_ident_double_quotes_token_stream
                            #first_token_stream
                            #second_token_stream
                        },
                    );
                    quote::quote! {
                        struct #postgresql_type_visitor_upper_camel_case;
                        #impl_serde_de_visitor_for_tokens_token_stream
                    }
                };
                (
                    generate_impl_serde_de_visitor_for_ident_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_interval_token_stream, &fn_visit_map_sqlx_postgres_types_pg_interval_token_stream),
                    generate_impl_serde_de_visitor_for_ident_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream),
                )
            };

            let (impl_serde_deserialize_for_field_sqlx_postgres_types_pg_interval_token_stream, impl_serde_deserialize_for_field_token_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_stream) = {
                let generate_impl_serde_deserialize_for_field_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        impl<'de> serde::Deserialize<'de> for Field {
                            fn deserialize<D>(__deserializer: D) -> Result<Field, D::Error>
                            where
                                D: serde::Deserializer<'de>,
                            {
                                #content_token_stream
                                #serde_deserializer_deserialize_identifier_token_stream
                            }
                        }
                    }
                };
                (
                    generate_impl_serde_deserialize_for_field_token_stream(&impl_serde_de_visitor_for_field_visitor_token_stream_31609291_37e6_427f_8d04_d19e2af929f8),
                    generate_impl_serde_deserialize_for_field_token_stream(&impl_serde_de_visitor_for_field_visitor_token_stream_ca843915_2330_4969_8bc8_8b33bff7a565),
                )
            };
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote! {
                    #field_start_end_token_stream
                    #impl_serde_deserialize_for_field_token_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_stream
                    #impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_ident_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote! {
                    #struct_visitor_token_stream
                    #impl_serde_de_visitor_for_visitor_uuid_uuid_token_stream
                    #serde_deserializer_deserialize_newtype_struct_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_types_bit_vec_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote! {
                    #struct_visitor_token_stream
                    #impl_serde_de_visitor_for_visitor_bit_vec_token_stream
                    #serde_deserializer_deserialize_newtype_struct_token_stream
                }
            });
            let generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream = |impl_serde_de_visitor_for_visitor_tokens_token_stream: &dyn quote::ToTokens| {
                generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                    #enum_field_two_token_stream
                    #impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    #impl_serde_de_visitor_for_visitor_tokens_token_stream
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                })
            };
            //////////////////////////////////////////
            let maybe_impl_serde_deserialize_for_postgresql_type_not_null_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_pg_money_token_stream
                            #serde_deserializer_deserialize_newtype_struct_token_stream
                        }
                    }),
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_8ae918a4_5464_4f56_8078_ab475f269079
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_types_big_decimal_token_stream
                            #const_fields_sqlx_types_big_decimal_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        }
                    }),
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlText => &proc_macro2_token_stream_new,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &proc_macro2_token_stream_new,
                    // default deserialize impl can cause an postgresql error "date of out range". pub const fn from_ordinal_date( do it too. if u want to check it just use sqlx::types::time::Date::MIN
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #enum_field_three_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_77c8b6d8_4ac3_4551_8498_36b9d77317f2
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_types_time_date_token_stream
                            #const_fields_sqlx_types_time_date_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        }
                    }),
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_mac_address_mac_address_token_stream
                            #serde_deserializer_deserialize_newtype_struct_token_stream
                        }
                    }),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &impl_serde_deserialize_for_sqlx_types_bit_vec_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &impl_serde_deserialize_for_sqlx_types_bit_vec_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #field_months_days_microseconds_token_stream
                            #impl_serde_deserialize_for_field_sqlx_postgres_types_pg_interval_token_stream
                            #impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_interval_token_stream
                            #const_fields_sqlx_postgres_types_pg_interval_token_stream
                            #serde_deserializer_deserialize_struct_ident_visitor_token_stream
                        }
                    }),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => {
                        &generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream)
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream
                            #const_fields_start_end_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        }
                    }),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => {
                        &generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream)
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => {
                        &generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream)
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => {
                        &generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream)
                    }
                },
            };
            let impl_std_fmt_display_for_postgresql_type_not_null_or_nullable_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(&postgresql_type_not_null_or_nullable_upper_camel_case, &quote::quote! {"{self:?}"});
            let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_not_null_or_nullable_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(&postgresql_type_not_null_or_nullable_upper_camel_case, &quote::quote! {self.to_string()});

            let sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream = quote::quote! {
                #sqlx_types_time_date_as_postgresql_date_field_type_token_stream::from_ordinal_date(
                    #core_default_default_default_token_stream,
                    1,
                ).unwrap()//todo
            };

            let sqlx_types_time_primitive_date_time_new_token_stream = quote::quote! {#sqlx_types_time_primitive_date_time_as_postgresql_timestamp_field_type_token_stream::new(
                #sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                #sqlx_types_time_time_midnight_token_stream,
            )};
            // fn std_net_ip_addr_v4_std_net_ipv4_addr_unspecified_token_stream() -> proc_macro2::TokenStream {
            //     quote::quote! {std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED)}
            // }

            let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_token_stream = match &postgresql_type_not_null_or_nullable {
                PostgresqlTypeNotNullOrNullable::NotNull => {
                    postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(&postgresql_type_not_null_or_nullable_upper_camel_case, &{
                        let generate_sqlx_postgres_types_pg_range_token_stream =
                            |start_token_stream: &dyn quote::ToTokens, end_token_stream: &dyn quote::ToTokens| generate_qlx_postgres_types_pg_range_start_end_token_stream(&quote::quote! {std::ops::Bound::Included(#start_token_stream)}, &quote::quote! {std::ops::Bound::Excluded(#end_token_stream)});
                        let sqlx_postgres_types_pg_range_core_default_default_default_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&core_default_default_default_token_stream, &core_default_default_default_token_stream);
                        let initialization_token_stream: &dyn quote::ToTokens = match &postgresql_type {
                            PostgresqlType::StdPrimitiveI16AsPostgresqlInt2
                            | PostgresqlType::StdPrimitiveI32AsPostgresqlInt4
                            | PostgresqlType::StdPrimitiveI64AsPostgresqlInt8
                            | PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4
                            | PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8
                            | PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql
                            | PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql
                            | PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &core_default_default_default_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &quote::quote! {#sqlx_postgres_types_pg_money_field_type_token_stream(#core_default_default_default_token_stream)},
                            PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric
                            | PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric
                            | PostgresqlType::StdPrimitiveBoolAsPostgresqlBool
                            | PostgresqlType::StdStringStringAsPostgresqlCharN
                            | PostgresqlType::StdStringStringAsPostgresqlVarchar
                            | PostgresqlType::StdStringStringAsPostgresqlText => &core_default_default_default_token_stream,
                            PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &quote::quote! {vec![#core_default_default_default_token_stream]},
                            PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &quote::quote! {#sqlx_types_time_time_midnight_token_stream},
                            PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &{
                                let double_dots_space_core_default_default_default_token_stream = generate_double_dot_space_tokens_token_stream(&core_default_default_default_token_stream);
                                generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(
                                    &double_dots_space_core_default_default_default_token_stream,
                                    &double_dots_space_core_default_default_default_token_stream,
                                    &double_dots_space_core_default_default_default_token_stream,
                                )
                            },
                            PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                            PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate | PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &core_default_default_default_token_stream,
                            PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &core_default_default_default_token_stream,
                            PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &sqlx_types_time_primitive_date_time_new_token_stream,
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz
                            | PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz
                            | PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql
                            | PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &core_default_default_default_token_stream,
                            PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &quote::quote! {
                                sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::new(core::net::Ipv4Addr::UNSPECIFIED, #core_default_default_default_token_stream).unwrap())
                            },
                            PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &core_default_default_default_token_stream,
                            PostgresqlType::SqlxTypesBitVecAsPostgresqlBit | PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &quote::quote! {{
                                let mut value = #sqlx_types_bit_vec_field_type_token_stream::new();
                                value.push(false);
                                value
                            }},
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &generate_sqlx_postgres_types_pg_range_token_stream(
                                &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                                &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                            ),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &generate_sqlx_postgres_types_pg_range_token_stream(&sqlx_types_time_primitive_date_time_new_token_stream, &sqlx_types_time_primitive_date_time_new_token_stream),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                        };
                        quote::quote! {Self(#initialization_token_stream)}
                    })
                }
                PostgresqlTypeNotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                    &postgresql_type_not_null_or_nullable_upper_camel_case,
                    &quote::quote! {Self(
                        Some(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
                    )},
                ),
            };
            let impl_sqlx_type_sqlx_postgres_for_postgresql_type_not_null_or_nullable_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(&postgresql_type_not_null_or_nullable_upper_camel_case, &field_type_handle);
            let impl_sqlx_encode_sqlx_postgres_for_postgresql_type_not_null_or_nullable_token_stream = generate_impl_sqlx_encode_sqlx_postgres_for_tokens_token_stream(&postgresql_type_not_null_or_nullable_upper_camel_case);
            let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_not_null_or_nullable_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(&postgresql_type_not_null_or_nullable_upper_camel_case, &field_type_handle);
            let impl_sqlx_postgres_pg_has_array_type_for_token_stream = quote::quote! {
                impl sqlx::postgres::PgHasArrayType for #postgresql_type_not_null_or_nullable_upper_camel_case {
                    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                        <#field_type as sqlx::postgres::PgHasArrayType>::array_type_info()
                    }
                }
            };
            let crate_try_generate_bind_increments_error_named_token_stream = crate_try_generate_bind_increments_error_named_token_stream();
            let impl_crate_bind_query_for_postgresql_type_not_null_or_nullable_token_stream = {
                let try_generate_bind_increments_token_stream = {
                    let increment_snake_case = naming::IncrementSnakeCase;
                    let acc_snake_case = naming::AccSnakeCase;
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("${{{increment_snake_case}}}"));
                    quote::quote! {
                        let mut #acc_snake_case = std::string::String::default();
                        match #increment_snake_case.checked_add(1) {
                            Some(#value_snake_case) => {
                                *#increment_snake_case = #value_snake_case;
                                #acc_snake_case.push_str(&format!(#format_handle_token_stream));
                            }
                            None => {
                                return Err(#crate_try_generate_bind_increments_error_named_token_stream::#checked_add_upper_camel_case {
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        }
                        Ok(#acc_snake_case)
                    }
                };
                match &postgresql_type_not_null_or_nullable {
                    PostgresqlTypeNotNullOrNullable::NotNull => generate_impl_crate_bind_query_for_tokens_token_stream(
                        &postgresql_type_not_null_or_nullable_upper_camel_case,
                        &try_generate_bind_increments_token_stream,
                        &quote::quote! {
                            #query_snake_case = #query_snake_case.bind(#self_snake_case);
                            #query_snake_case
                        },
                    ),
                    PostgresqlTypeNotNullOrNullable::Nullable => generate_impl_crate_bind_query_for_tokens_token_stream(
                        &postgresql_type_not_null_or_nullable_upper_camel_case,
                        &try_generate_bind_increments_token_stream,
                        &quote::quote! {
                            #query_snake_case = #query_snake_case.bind(match #self_dot_zero_token_stream {
                                Some(#value_snake_case) => Some(#value_snake_case),
                                None => None
                            });
                            #query_snake_case
                        },
                    ),
                }
            };
            enum CanBePrimaryKey {
                True,
                False,
            }
            let can_be_primary_key = match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => CanBePrimaryKey::False,
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => CanBePrimaryKey::False,
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => CanBePrimaryKey::False,
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => CanBePrimaryKey::False,
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => CanBePrimaryKey::False,
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => CanBePrimaryKey::True,
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => CanBePrimaryKey::True,
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => CanBePrimaryKey::True,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => CanBePrimaryKey::False,
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => CanBePrimaryKey::False,
                PostgresqlType::StdStringStringAsPostgresqlCharN => CanBePrimaryKey::False,
                PostgresqlType::StdStringStringAsPostgresqlVarchar => CanBePrimaryKey::False,
                PostgresqlType::StdStringStringAsPostgresqlText => CanBePrimaryKey::False,
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => CanBePrimaryKey::True,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => CanBePrimaryKey::False,
            };
            let impl_crate_create_table_column_query_part_for_postgresql_type_not_null_or_nullable_token_stream = {
                let fixed_length_snake_case = naming::FixedLengthSnakeCase;
                let postgresql_query_type = match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => "int2",
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => "int4",
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => "int8",
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => "float4",
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => "float8",
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => "smallserial",
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => "serial",
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => "bigserial",
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => "money",
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => "numeric",
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => "numeric",
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => "bool",
                    PostgresqlType::StdStringStringAsPostgresqlCharN => &format!("char({{{fixed_length_snake_case}}})"),
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => &format!("varchar({{{fixed_length_snake_case}}})"),
                    PostgresqlType::StdStringStringAsPostgresqlText => "text",
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => "bytea",
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => "time",
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => "time",
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => "interval",
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => "date",
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => "date",
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => "timestamp",
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => "timestamp",
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => "timestamptz",
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => "timestamptz",
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => "uuid",
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => "uuid",
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => "inet",
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => "cidr",
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => "macaddr",
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &format!("bit({{{fixed_length_snake_case}}})"),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &format!("bit varying({{{fixed_length_snake_case}}})"),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => "int4range",
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => "int8range",
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => "numrange",
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => "numrange",
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => "daterange",
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => "daterange",
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => "tsrange",
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => "tsrange",
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => "tstzrange",
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => "tstzrange",
                };
                let crate_maybe_primary_key_is_primary_key_token_stream = quote::quote! {crate::maybe_primary_key(is_primary_key)};
                let column_postgresql_query_type = format!("{{column}} {postgresql_query_type}");
                let column_postgresql_query_type_not_null = format!("{column_postgresql_query_type} not null");
                let space_additional_parameter = " {}";
                let content_token_stream = match (&postgresql_type_not_null_or_nullable, &can_be_primary_key) {
                    (PostgresqlTypeNotNullOrNullable::NotNull, CanBePrimaryKey::False) => {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&column_postgresql_query_type_not_null.to_string());
                        quote::quote! {
                            format!(#format_handle_token_stream)
                        }
                    }
                    (PostgresqlTypeNotNullOrNullable::NotNull, CanBePrimaryKey::True) => {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{column_postgresql_query_type_not_null}{space_additional_parameter}"));
                        quote::quote! {
                            format!(#format_handle_token_stream, #crate_maybe_primary_key_is_primary_key_token_stream)
                        }
                    }
                    (PostgresqlTypeNotNullOrNullable::Nullable, CanBePrimaryKey::False) => {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&column_postgresql_query_type);
                        quote::quote! {
                            format!(#format_handle_token_stream)
                        }
                    }
                    (PostgresqlTypeNotNullOrNullable::Nullable, CanBePrimaryKey::True) => {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{column_postgresql_query_type}{space_additional_parameter}"));
                        quote::quote! {
                            format!(#format_handle_token_stream, #crate_maybe_primary_key_is_primary_key_token_stream)
                        }
                    }
                };
                let fixed_length_parameter_token_stream = {
                    let postgresql_type_length_upper_camel_case = naming::parameter::SelfLengthUpperCamelCase::from_tokens(&postgresql_type);
                    quote::quote! {, #fixed_length_snake_case: #postgresql_type_length_upper_camel_case}
                };
                let maybe_fixed_length_parameter_token_stream: &dyn quote::ToTokens = match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => &fixed_length_parameter_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => &fixed_length_parameter_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlText => &proc_macro2_token_stream_new,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &fixed_length_parameter_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &fixed_length_parameter_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                };
                quote::quote! {
                    impl #postgresql_type_not_null_or_nullable_upper_camel_case {
                        pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool #maybe_fixed_length_parameter_token_stream) -> impl std::fmt::Display {
                            #content_token_stream
                        }
                    }
                }
            };
            let postgresql_type_not_null_or_nullable_column_upper_camel_case = naming::parameter::SelfColumnUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
            let postgresql_type_not_null_or_nullable_column_token_stream = {
                let pub_struct_postgresql_type_not_null_or_nullable_column_token_stream = generate_pub_struct_tokens_token_stream(&postgresql_type_not_null_or_nullable_column_upper_camel_case, &quote::quote! {;}, true, true);
                let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_column_token_stream =
                    postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                        &postgresql_type_not_null_or_nullable_column_upper_camel_case,
                        &core_default_default_default_token_stream,
                    );
                quote::quote! {
                    #pub_struct_postgresql_type_not_null_or_nullable_column_token_stream
                    #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_column_token_stream
                }
            };
            let postgresql_type_not_null_or_nullable_to_create_upper_camel_case = naming::parameter::SelfToCreateUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
            let try_generate_bind_increments_snake_case = naming::TryGenerateBindIncrementsSnakeCase;
            let crate_bind_query_token_stream = crate_bind_query_token_stream();
            let bind_value_to_query_snake_case = naming::BindValueToQuerySnakeCase;
            let postgresql_type_not_null_or_nullable_to_create_token_stream = {
                let generate_initialized_by_postgresql_token_stream = |query_part_token_stream: &dyn quote::ToTokens| {
                    let postgresql_type_not_null_or_nullable_to_create_token_stream = generate_pub_struct_tokens_token_stream(&postgresql_type_not_null_or_nullable_to_create_upper_camel_case, &quote::quote! {(());}, false, true);
                    let impl_crate_bind_query_for_postgresql_type_not_null_or_nullable_to_create_token_stream =
                        generate_impl_crate_bind_query_for_tokens_token_stream(&postgresql_type_not_null_or_nullable_to_create_upper_camel_case, &quote::quote! {Ok(#std_string_string_token_stream::from(#query_part_token_stream))}, &query_snake_case);
                    let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_to_create_token_stream =
                        postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                            &postgresql_type_not_null_or_nullable_to_create_upper_camel_case,
                            &quote::quote! {Self(#core_default_default_default_token_stream)},
                        );
                    quote::quote! {
                        #postgresql_type_not_null_or_nullable_to_create_token_stream
                        #impl_crate_bind_query_for_postgresql_type_not_null_or_nullable_to_create_token_stream
                        #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_to_create_token_stream
                    }
                };
                let default_initialized_by_postgresql_token_stream = generate_initialized_by_postgresql_token_stream(&quote::quote! {"DEFAULT"});
                let uuid_generate_v4_initialized_by_postgresql_token_stream = generate_initialized_by_postgresql_token_stream(&quote::quote! {"uuid_generate_v4()"});
                let alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_type_not_null_or_nullable_to_create_upper_camel_case, &postgresql_type_not_null_or_nullable_upper_camel_case);
                match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => alias_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => alias_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => alias_token_stream,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => alias_token_stream,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => alias_token_stream,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => default_initialized_by_postgresql_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => default_initialized_by_postgresql_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => default_initialized_by_postgresql_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => alias_token_stream,
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => alias_token_stream,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => alias_token_stream,
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => alias_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => alias_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => alias_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlText => alias_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => alias_token_stream,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => alias_token_stream,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => alias_token_stream,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => alias_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => uuid_generate_v4_initialized_by_postgresql_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => alias_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => alias_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => alias_token_stream,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => alias_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => alias_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => alias_token_stream,
                }
            };
            let postgresql_type_not_null_or_nullable_to_read_upper_camel_case = naming::parameter::SelfToReadUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
            let postgresql_type_not_null_or_nullable_to_read_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_type_not_null_or_nullable_to_read_upper_camel_case, &postgresql_type_not_null_or_nullable_upper_camel_case);
            let postgresql_type_not_null_or_nullable_to_update_upper_camel_case = naming::parameter::SelfToUpdateUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
            let postgresql_type_not_null_or_nullable_to_update_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_type_not_null_or_nullable_to_update_upper_camel_case, &postgresql_type_not_null_or_nullable_upper_camel_case);
            let postgresql_type_not_null_or_nullable_upper_camel_case_to_delete_upper_camel_case = naming::parameter::SelfToDeleteUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
            let postgresql_type_not_null_or_nullable_upper_camel_case_to_delete_token_stream =
                macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_type_not_null_or_nullable_upper_camel_case_to_delete_upper_camel_case, &postgresql_type_not_null_or_nullable_upper_camel_case);
            //todo put delete into trait
            // let postgresql_type_not_null_or_nullable_to_delete_token_stream = {
            //     let postgresql_type_not_null_or_nullable_to_delete_token_stream = generate_pub_struct_tokens_token_stream(
            //         &postgresql_type_not_null_or_nullable_to_delete_upper_camel_case,
            //         &postgresql_type_not_null_or_nullable_struct_content_token_stream,
            //         false,
            //         true,
            //     );
            //     let impl_crate_bind_query_for_postgresql_type_not_null_or_nullable_to_delete_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
            //         &postgresql_type_not_null_or_nullable_to_delete_upper_camel_case,
            //         &crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream,
            //         &crate_bind_query_bind_value_to_query_self_zero_query_token_stream,
            //     );
            //     let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_to_delete_token_stream = postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            //         &postgresql_type_not_null_or_nullable_to_delete_upper_camel_case,
            //         &self_braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            //     );
            //     let impl_std_fmt_display_for_postgresql_type_not_null_or_nullable_to_delete_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(
            //         &postgresql_type_not_null_or_nullable_to_delete_upper_camel_case,
            //         &impl_std_fmt_display_for_tokens_self_zero_content_token_stream
            //     );
            //     let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_not_null_or_nullable_to_delete_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(
            //         &postgresql_type_not_null_or_nullable_to_delete_upper_camel_case,
            //         &quote::quote!{format!("{self}")}
            //     );
            //     let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_not_null_or_nullable_to_delete_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
            //         &postgresql_type_not_null_or_nullable_to_delete_upper_camel_case,
            //         &field_type_handle
            //     );
            //     let impl_sqlx_type_sqlx_postgres_for_postgresql_type_not_null_or_nullable_to_delete_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
            //         &postgresql_type_not_null_or_nullable_to_delete_upper_camel_case,
            //         &field_type_handle
            //     );
            //     quote::quote!{
            //         #postgresql_type_not_null_or_nullable_to_delete_token_stream
            //         #impl_std_fmt_display_for_postgresql_type_not_null_or_nullable_to_delete_token_stream
            //         #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_not_null_or_nullable_to_delete_token_stream
            //         #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_not_null_or_nullable_to_delete_token_stream
            //         #impl_sqlx_type_sqlx_postgres_for_postgresql_type_not_null_or_nullable_to_delete_token_stream
            //         #impl_crate_bind_query_for_postgresql_type_not_null_or_nullable_to_delete_token_stream
            //         #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_to_delete_token_stream
            //     }
            // };

            let postgresql_type_not_null_or_nullable_where_element_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
            let postgresql_type_not_null_or_nullable_where_element_token_stream = {
                let where_operator_type_field_type_default = WhereOperatorType::FieldType {
                    field_type: &postgresql_type_not_null_or_nullable_upper_camel_case,
                    default_initialization_token_stream: &crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                };
                let where_operator_type_ident = WhereOperatorType::Ident(&postgresql_type_not_null_upper_camel_case);
                let generate_postgresql_type_not_null_or_nullable_where_element_token_stream = |variants: &std::vec::Vec<&dyn crate::filters::WhereOperatorName>| {
                    generate_postgresql_type_where_element_token_stream(
                        variants,
                        &postgresql_type_not_null_or_nullable_where_element_upper_camel_case,
                        &naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_display(&postgresql_type_not_null_upper_camel_case),
                        &ShouldDeriveSchemarsJsonSchema::False,
                    )
                };

                let equal = crate::filters::Equal;
                let greater_than = crate::filters::GreaterThan;
                let between = crate::filters::Between;
                let in_handle = crate::filters::In;
                let case_sensitive_regular_expression = crate::filters::CaseSensitiveRegularExpression;
                let case_insensitive_regular_expression = crate::filters::CaseInsensitiveRegularExpression;
                let length_more_than = crate::filters::LengthMoreThan;
                let equal_to_encoded_string_representation = crate::filters::EqualToEncodedStringRepresentation;
                let current_date = crate::filters::CurrentDate;
                let greater_than_current_date = crate::filters::GreaterThanCurrentDate;
                let current_time = crate::filters::CurrentTime;
                let greater_than_current_time = crate::filters::GreaterThanCurrentTime;
                let value_is_contained_within_range = crate::filters::ValueIsContainedWithinRange;
                let contains_another_range = crate::filters::ContainsAnotherRange;
                let strictly_to_left_of_range = crate::filters::StrictlyToLeftOfRange;
                let strictly_to_right_of_range = crate::filters::StrictlyToRightOfRange;
                let included_lower_bound = crate::filters::IncludedLowerBound;
                let excluded_upper_bound = crate::filters::ExcludedUpperBound;
                let greater_than_lower_bound = crate::filters::GreaterThanLowerBound;
                let overlap_with_range = crate::filters::OverlapWithRange;
                let adjacent_with_range = crate::filters::AdjacentWithRange;
                let range_length = crate::filters::RangeLength;
                let current_timestamp = crate::filters::CurrentTimestamp;
                let greater_than_current_timestamp = crate::filters::GreaterThanCurrentTimestamp;
                let before = crate::filters::Before;
                let bit_vec_position_equal = crate::filters::BitVecPositionEqual;

                let postgresql_type_tokens_where_element_equal_where_operator_type_field_type_default_token_stream =
                    equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, postgresql_type_not_null_or_nullable, &where_operator_type_field_type_default);
                let postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, postgresql_type_not_null_or_nullable, &where_operator_type_ident);

                let postgresql_type_tokens_where_element_greater_than_where_operator_type_field_type_default_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_field_type_default);
                let postgresql_type_tokens_where_element_greater_than_where_operator_type_ident_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_ident);

                let where_element_number_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                                &postgresql_type_not_null_upper_camel_case,
                                &where_operator_type_field_type_default,
                                &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                                &crate::filters::ShouldAddDotZero::False,
                            );
                            let postgresql_type_tokens_where_element_in_token_stream = in_handle.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_field_type_default);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_where_operator_type_field_type_default_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                                #postgresql_type_tokens_where_element_in_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &in_handle]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_field_type_default_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_postgres_types_pg_money_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_between_token_stream =
                                between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_ident, &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd, &crate::filters::ShouldAddDotZero::True);
                            let postgresql_type_tokens_where_element_in_token_stream = in_handle.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_ident);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_where_operator_type_ident_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                                #postgresql_type_tokens_where_element_in_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &in_handle]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_decimal_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                                &postgresql_type_not_null_upper_camel_case,
                                &where_operator_type_field_type_default,
                                &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                                &crate::filters::ShouldAddDotZero::False,
                            );
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_where_operator_type_field_type_default_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_field_type_default_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_big_decimal_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_between_token_stream =
                                between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_ident, &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd, &crate::filters::ShouldAddDotZero::False);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_where_operator_type_ident_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_bool_token_stream = {
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_field_type_default_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_std_string_string_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream
                                #postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &case_sensitive_regular_expression, &case_insensitive_regular_expression]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_field_type_default_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_std_vec_vec_std_primitive_u8_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_length_more_than_token_stream = length_more_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_equal_to_encoded_string_representation_token_stream = equal_to_encoded_string_representation.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_length_more_than_token_stream
                                #postgresql_type_tokens_where_element_equal_to_encoded_string_representation_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &length_more_than, &equal_to_encoded_string_representation]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_field_type_default_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_time_date_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_between_token_stream =
                                between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_ident, &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd, &crate::filters::ShouldAddDotZero::False);
                            let postgresql_type_tokens_where_element_current_date_token_stream = current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_greater_than_current_date_token_stream = greater_than_current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_where_operator_type_ident_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                                #postgresql_type_tokens_where_element_current_date_token_stream
                                #postgresql_type_tokens_where_element_greater_than_current_date_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_date, &greater_than_current_date]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_chrono_naive_date_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                                &postgresql_type_not_null_upper_camel_case,
                                &where_operator_type_field_type_default,
                                &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                                &crate::filters::ShouldAddDotZero::False,
                            );
                            let postgresql_type_tokens_where_element_current_date_token_stream = current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_greater_than_current_date_token_stream = greater_than_current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_where_operator_type_field_type_default_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                                #postgresql_type_tokens_where_element_current_date_token_stream
                                #postgresql_type_tokens_where_element_greater_than_current_date_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_date, &greater_than_current_date]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_field_type_default_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_chrono_naive_time_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                                &postgresql_type_not_null_upper_camel_case,
                                &where_operator_type_field_type_default,
                                &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                                &crate::filters::ShouldAddDotZero::False,
                            );
                            let postgresql_type_tokens_where_element_current_time_token_stream = current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_greater_than_current_time_token_stream = greater_than_current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_where_operator_type_field_type_default_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                                #postgresql_type_tokens_where_element_current_time_token_stream
                                #postgresql_type_tokens_where_element_greater_than_current_time_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_time, &greater_than_current_time]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_field_type_default_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_time_time_token_stream = {
                    let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, postgresql_type_not_null_or_nullable, &where_operator_type_field_type_default);
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_field_type_default);
                            let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                                &postgresql_type_not_null_upper_camel_case,
                                &where_operator_type_field_type_default,
                                &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                                &crate::filters::ShouldAddDotZero::False,
                            );
                            let postgresql_type_tokens_where_element_current_time_token_stream = current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_greater_than_current_time_token_stream = greater_than_current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                                #postgresql_type_tokens_where_element_current_time_token_stream
                                #postgresql_type_tokens_where_element_greater_than_current_time_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_time, &greater_than_current_time]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_postgres_types_pg_interval_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_between_token_stream =
                                between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_ident, &crate::filters::BetweenTryNewErrorType::StartIsEqualToEnd, &crate::filters::ShouldAddDotZero::False);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_where_operator_type_ident_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream = |range_type: RangeType| {
                    enum ShouldImplRangeLength {
                        True,
                        False,
                    }
                    let range_type_should_impl_range_length = {
                        let should_impl_range_length_true = ShouldImplRangeLength::True;
                        let should_impl_range_length_false = ShouldImplRangeLength::False;
                        match &range_type {
                            RangeType::StdPrimitiveI32AsPostgresqlInt4 => should_impl_range_length_true,
                            RangeType::StdPrimitiveI64AsPostgresqlInt8 => should_impl_range_length_true,
                            RangeType::SqlxTypesDecimalAsPostgresqlNumeric => should_impl_range_length_false,
                            RangeType::SqlxTypesBigDecimalAsPostgresqlNumeric => should_impl_range_length_false,
                            RangeType::SqlxTypesTimeDateAsPostgresqlDate => should_impl_range_length_false,
                            RangeType::SqlxTypesChronoNaiveDateAsPostgresqlDate => should_impl_range_length_false,
                            RangeType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => should_impl_range_length_false,
                            RangeType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => should_impl_range_length_false,
                            RangeType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => should_impl_range_length_false,
                            RangeType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => should_impl_range_length_false,
                        }
                    };
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let dot_zero_token_stream = quote::quote! {.0};
                            let postgresql_type_tokens_where_element_value_is_contained_within_range_token_stream = value_is_contained_within_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                                &postgresql_type_not_null_upper_camel_case,
                                &range_type,
                                &crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                                &dot_zero_token_stream,
                            );
                            let postgresql_type_tokens_where_element_contains_another_range_token_stream = contains_another_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_strictly_to_left_of_range_token_stream = strictly_to_left_of_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_strictly_to_right_of_range_token_stream = strictly_to_right_of_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_included_lower_bound_token_stream = included_lower_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                                &postgresql_type_not_null_upper_camel_case,
                                &range_type,
                                &crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                                &dot_zero_token_stream,
                            );
                            let postgresql_type_tokens_where_element_excluded_upper_bound_token_stream = excluded_upper_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                                &postgresql_type_not_null_upper_camel_case,
                                &range_type,
                                &crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                                &dot_zero_token_stream,
                            );
                            let postgresql_type_tokens_where_element_greater_than_lower_bound_token_stream = greater_than_lower_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_overlap_with_range_token_stream = overlap_with_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_adjacent_with_range_token_stream = adjacent_with_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let maybe_postgresql_type_tokens_where_element_range_length_token_stream: &dyn quote::ToTokens = match &range_type_should_impl_range_length {
                                ShouldImplRangeLength::True => &range_length.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case),
                                ShouldImplRangeLength::False => &proc_macro2_token_stream_new,
                            };
                            quote::quote! {
                                #postgresql_type_tokens_where_element_value_is_contained_within_range_token_stream
                                #postgresql_type_tokens_where_element_contains_another_range_token_stream
                                #postgresql_type_tokens_where_element_strictly_to_left_of_range_token_stream
                                #postgresql_type_tokens_where_element_strictly_to_right_of_range_token_stream
                                #postgresql_type_tokens_where_element_included_lower_bound_token_stream
                                #postgresql_type_tokens_where_element_excluded_upper_bound_token_stream
                                #postgresql_type_tokens_where_element_greater_than_lower_bound_token_stream
                                #postgresql_type_tokens_where_element_overlap_with_range_token_stream
                                #postgresql_type_tokens_where_element_adjacent_with_range_token_stream
                                #maybe_postgresql_type_tokens_where_element_range_length_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&{
                        let mut value: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = vec![
                            &equal,
                            &value_is_contained_within_range,
                            &contains_another_range,
                            &strictly_to_left_of_range,
                            &strictly_to_right_of_range,
                            &included_lower_bound,
                            &excluded_upper_bound,
                            &greater_than_lower_bound,
                            &overlap_with_range,
                            &adjacent_with_range,
                        ];
                        if let ShouldImplRangeLength::True = &range_type_should_impl_range_length {
                            value.push(&range_length);
                        }
                        value
                    });
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };

                let where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::StdPrimitiveI32AsPostgresqlInt4);
                let where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::StdPrimitiveI64AsPostgresqlInt8);
                let where_element_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxTypesDecimalAsPostgresqlNumeric);
                let where_element_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxTypesBigDecimalAsPostgresqlNumeric);
                let where_element_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxTypesTimeDateAsPostgresqlDate);
                let where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxTypesChronoNaiveDateAsPostgresqlDate);
                let where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp);
                let where_element_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp);
                let where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz);
                let where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz);
                let where_element_sqlx_types_chrono_naive_date_time_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                                &postgresql_type_not_null_upper_camel_case,
                                &where_operator_type_field_type_default,
                                &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                                &crate::filters::ShouldAddDotZero::False,
                            );
                            let postgresql_type_tokens_where_element_current_timestamp_token_stream = current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream = greater_than_current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_where_operator_type_field_type_default_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                                #postgresql_type_tokens_where_element_current_timestamp_token_stream
                                #postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_timestamp, &greater_than_current_timestamp]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_field_type_default_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_time_primitive_date_time_token_stream = {
                    let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, postgresql_type_not_null_or_nullable, &where_operator_type_field_type_default);
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_field_type_default);
                            let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                                &postgresql_type_not_null_upper_camel_case,
                                &where_operator_type_field_type_default,
                                &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                                &crate::filters::ShouldAddDotZero::False,
                            );
                            let postgresql_type_tokens_where_element_current_timestamp_token_stream = current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream = greater_than_current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                                #postgresql_type_tokens_where_element_current_timestamp_token_stream
                                #postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_timestamp, &greater_than_current_timestamp]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_before_token_stream = before.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_between_token_stream =
                                between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_ident, &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd, &crate::filters::ShouldAddDotZero::False);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_before_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &before, &between]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_before_token_stream = before.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_between_token_stream =
                                between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, &where_operator_type_ident, &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd, &crate::filters::ShouldAddDotZero::False);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_before_token_stream
                                #postgresql_type_tokens_where_element_between_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &before, &between]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_uuid_uuid_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream
                                #postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &case_sensitive_regular_expression, &case_insensitive_regular_expression]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_ipnetwork_ip_network_token_stream = {
                    let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case, postgresql_type_not_null_or_nullable, &where_operator_type_field_type_default);
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_mac_address_mac_address_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_greater_than_where_operator_type_ident_token_stream
                                #postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream
                                #postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &case_sensitive_regular_expression, &case_insensitive_regular_expression]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                let where_element_sqlx_types_bit_vec_token_stream = {
                    let maybe_filters_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                        PostgresqlTypeNotNullOrNullable::NotNull => &{
                            let postgresql_type_tokens_where_element_bit_vec_position_equal_token_stream = bit_vec_position_equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&postgresql_type_not_null_upper_camel_case);
                            quote::quote! {
                                #postgresql_type_tokens_where_element_bit_vec_position_equal_token_stream
                            }
                        },
                        PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &bit_vec_position_equal]);
                    quote::quote! {
                        #postgresql_type_tokens_where_element_equal_where_operator_type_ident_token_stream
                        #maybe_filters_token_stream
                        #postgresql_type_tokens_where_element_token_stream
                    }
                };
                match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => where_element_number_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => where_element_number_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => where_element_number_token_stream,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => where_element_number_token_stream,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => where_element_number_token_stream,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => where_element_number_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => where_element_number_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => where_element_number_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => where_element_sqlx_postgres_types_pg_money_token_stream,
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => where_element_sqlx_types_decimal_token_stream,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => where_element_sqlx_types_big_decimal_token_stream,
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => where_element_bool_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => where_element_std_string_string_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => where_element_std_string_string_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlText => where_element_std_string_string_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => where_element_std_vec_vec_std_primitive_u8_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => where_element_sqlx_types_chrono_naive_time_token_stream,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => where_element_sqlx_types_time_time_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => where_element_sqlx_postgres_types_pg_interval_token_stream,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => where_element_sqlx_types_time_date_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => where_element_sqlx_types_chrono_naive_date_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => where_element_sqlx_types_chrono_naive_date_time_token_stream,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => where_element_sqlx_types_time_primitive_date_time_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => where_element_sqlx_types_uuid_uuid_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => where_element_sqlx_types_uuid_uuid_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => where_element_sqlx_types_ipnetwork_ip_network_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => where_element_sqlx_types_ipnetwork_ip_network_token_stream,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => where_element_sqlx_types_mac_address_mac_address_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => where_element_sqlx_types_bit_vec_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => where_element_sqlx_types_bit_vec_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                }
            };
            let postgresql_type_not_null_or_nullable_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
            //todo maybe put Where into library? seems like it can be struct with generic parameter what can be imported from library
            let postgresql_type_not_null_or_nullable_where_token_stream = generate_postgresql_type_or_json_type_where_token_stream(
                &PostgresqlTypeOrJsonType::PostgresqlType,
                &postgresql_type_not_null_or_nullable_upper_camel_case,
                &postgresql_type_not_null_or_nullable_where_element_upper_camel_case,
                &postgresql_type_not_null_or_nullable_where_upper_camel_case,
            );

            let impl_postgresql_type_for_ident_token_stream = {
                let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
                let self_upper_camel_case = naming::SelfUpperCamelCase;
                let postgresql_type_self_upper_camel_case = naming::PostgresqlTypeSelfUpperCamelCase;
                let create_upper_camel_case = naming::CreateUpperCamelCase;
                let read_upper_camel_case = naming::ReadUpperCamelCase;
                let self_where_element_upper_camel_case = naming::SelfWhereElementUpperCamelCase;
                let column_upper_camel_case = naming::ColumnUpperCamelCase;
                let self_column_query_part_token_stream = {
                    let self_column_snake_case = naming::SelfColumnSnakeCase;
                    quote::quote! {
                        fn column_query_part(
                            #self_column_snake_case: &Self::#column_upper_camel_case,
                            #column_snake_case: &std::primitive::str,
                        ) -> #std_string_string_token_stream {
                            #column_snake_case.to_string()
                        }
                    }
                };
                let self_to_update_upper_camel_case = naming::SelfToUpdateUpperCamelCase;
                let self_to_update_query_part_error_named_upper_camel_case = naming::SelfToUpdateQueryPartErrorNamedUpperCamelCase;
                let postgresql_type_self_to_update_query_part_token_stream = {
                    let self_to_update_query_part_snake_case = naming::SelfToUpdateQueryPartSnakeCase;
                    let self_to_update_snake_case = naming::SelfToUpdateSnakeCase;
                    quote::quote! {
                        fn #self_to_update_query_part_snake_case(
                            #self_to_update_snake_case: &Self::#self_to_update_upper_camel_case,
                            _: &std::primitive::str,
                            _: &std::primitive::str,
                            _: &std::primitive::str,
                            #increment_snake_case: &mut std::primitive::u64
                        ) -> Result<#std_string_string_token_stream, Self::#self_to_update_query_part_error_named_upper_camel_case> {
                            #crate_bind_query_token_stream #try_generate_bind_increments_snake_case(#self_to_update_snake_case, #increment_snake_case)
                        }
                    }
                };
                let self_to_update_bind_query_part_token_stream = {
                    let self_to_update_bind_query_part = naming::SelfToUpdateBindQueryPartSnakeCase;
                    let self_to_update_snake_case = naming::SelfToUpdateSnakeCase;
                    quote::quote! {
                        fn #self_to_update_bind_query_part<'a>(
                            #self_to_update_snake_case: Self::#self_to_update_upper_camel_case,
                            #query_snake_case: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                        ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                            #crate_bind_query_token_stream #bind_value_to_query_snake_case(#self_to_update_snake_case, #query_snake_case)
                        }
                    }
                };
                let self_where_upper_camel_case = naming::SelfWhereUpperCamelCase;
                let self_where_snake_case = naming::SelfWhereSnakeCase;
                let postgresql_type_self_where_try_generate_bind_increments_token_stream = {
                    let self_where_try_generate_bind_increments_snake_case = naming::SelfWhereTryGenerateBindIncrementsSnakeCase;
                    quote::quote! {
                        fn #self_where_try_generate_bind_increments_snake_case(
                            #self_where_snake_case: &Self::#self_where_upper_camel_case,
                            #increment_snake_case: &mut std::primitive::u64,
                            #column_snake_case: &dyn std::fmt::Display,
                            is_need_to_add_logical_operator: std::primitive::bool,
                        ) -> Result<#std_string_string_token_stream, crate::TryGenerateBindIncrementsErrorNamed> {
                            let mut #acc_snake_case = #std_string_string_token_stream::default();
                            let mut is_need_to_add_logical_operator_inner_handle = false;
                            for #element_snake_case in &#self_where_snake_case.#value_snake_case {
                                match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::self_where_try_generate_bind_increments(
                                    #element_snake_case,
                                    #increment_snake_case,
                                    #column_snake_case,
                                    is_need_to_add_logical_operator_inner_handle
                                ) {
                                    Ok(#value_snake_case) => {
                                        #acc_snake_case.push_str(&format!("{value} "));
                                        is_need_to_add_logical_operator_inner_handle = true;
                                    }
                                    Err(#error_snake_case) => {
                                        return Err(#error_snake_case);
                                    }
                                }
                            }
                            let _ = #acc_snake_case.pop();
                            Ok(format!("{}({acc})", &#self_where_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                        }
                    }
                };
                let postgresql_type_self_where_bind_value_to_query_token_stream = {
                    let self_where_bind_value_to_query_snake_case = naming::SelfWhereBindValueToQuerySnakeCase;
                    quote::quote! {
                        fn #self_where_bind_value_to_query_snake_case<'a>(
                            #self_where_snake_case: Self::#self_where_upper_camel_case,
                            mut #query_snake_case: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                        ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                            for #element_snake_case in #self_where_snake_case.#value_snake_case {
                                #query_snake_case = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::self_where_bind_value_to_query(#element_snake_case, #query_snake_case);
                            }
                            #query_snake_case
                        }
                    }
                };
                quote::quote! {
                    impl crate::postgresql_type::postgresql_type_trait:: #postgresql_type_upper_camel_case<'_> for #postgresql_type_not_null_or_nullable_upper_camel_case {
                        type #postgresql_type_self_upper_camel_case = #self_upper_camel_case;
                        type #column_upper_camel_case = #postgresql_type_not_null_or_nullable_column_upper_camel_case;
                        #self_column_query_part_token_stream
                        type #create_upper_camel_case = #postgresql_type_not_null_or_nullable_to_create_upper_camel_case;
                        type #read_upper_camel_case = #postgresql_type_not_null_or_nullable_to_read_upper_camel_case;
                        type #self_to_update_upper_camel_case = #postgresql_type_not_null_or_nullable_to_update_upper_camel_case;
                        type #self_to_update_query_part_error_named_upper_camel_case = #crate_try_generate_bind_increments_error_named_token_stream;
                        #postgresql_type_self_to_update_query_part_token_stream
                        #self_to_update_bind_query_part_token_stream
                        type #self_where_element_upper_camel_case = #postgresql_type_not_null_or_nullable_where_element_upper_camel_case;
                        type #self_where_upper_camel_case = #postgresql_type_not_null_or_nullable_where_upper_camel_case;
                        #postgresql_type_self_where_try_generate_bind_increments_token_stream
                        #postgresql_type_self_where_bind_value_to_query_token_stream
                    }
                }
            };

            ////////////////////////////////////////
            // let vec_postgresql_type_array_not_null_upper_camel_case = naming::parameter::VecSelfArrayNotNullUpperCamelCase::from_tokens(&postgresql_type);
            // let vec_postgresql_type_array_not_null_or_nullable_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_not_null_or_nullable {
            //     PostgresqlTypeNotNullOrNullable::NotNull => &vec_postgresql_type_array_not_null_upper_camel_case,
            //     PostgresqlTypeNotNullOrNullable::Nullable => &naming::parameter::VecSelfArrayNullableUpperCamelCase::from_tokens(&postgresql_type),
            // };
            // let field_type_handle: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
            //     PostgresqlTypeNotNullOrNullable::NotNull => &field_type,
            //     PostgresqlTypeNotNullOrNullable::Nullable => &quote::quote! {std::option::Option<#postgresql_type_not_null_upper_camel_case>},
            // };
            // let vec_field_type_handle = quote::quote! {std::vec::Vec<#postgresql_type_not_null_upper_camel_case>}
            quote::quote! {
                #pub_struct_postgresql_type_not_null_or_nullable_token_stream
                #maybe_impl_try_new_for_postgresql_type_not_null_token_stream
                #maybe_impl_serde_serialize_for_postgresql_type_not_null_token_stream
                #maybe_impl_serde_deserialize_for_postgresql_type_not_null_token_stream
                #impl_std_fmt_display_for_postgresql_type_not_null_or_nullable_token_stream
                #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_not_null_or_nullable_token_stream
                #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_token_stream
                #impl_sqlx_type_sqlx_postgres_for_postgresql_type_not_null_or_nullable_token_stream
                #impl_sqlx_encode_sqlx_postgres_for_postgresql_type_not_null_or_nullable_token_stream
                #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_not_null_or_nullable_token_stream
                #impl_sqlx_postgres_pg_has_array_type_for_token_stream
                #impl_crate_bind_query_for_postgresql_type_not_null_or_nullable_token_stream
                #impl_crate_create_table_column_query_part_for_postgresql_type_not_null_or_nullable_token_stream
                #postgresql_type_not_null_or_nullable_column_token_stream
                #postgresql_type_not_null_or_nullable_to_create_token_stream
                #postgresql_type_not_null_or_nullable_to_read_token_stream
                #postgresql_type_not_null_or_nullable_to_update_token_stream
                #postgresql_type_not_null_or_nullable_upper_camel_case_to_delete_token_stream
                #postgresql_type_not_null_or_nullable_where_element_token_stream
                #postgresql_type_not_null_or_nullable_where_token_stream
                #impl_postgresql_type_for_ident_token_stream
            }
        };
        let postgresql_type_not_null_token_stream = generate_postgresql_type_not_null_or_nullable(&PostgresqlTypeNotNullOrNullable::NotNull);
        let maybe_postgresql_type_nullable_token_stream: &dyn quote::ToTokens = match &postgresql_type.can_be_nullable() {
            CanBeNullable::True => &generate_postgresql_type_not_null_or_nullable(&PostgresqlTypeNotNullOrNullable::Nullable),
            CanBeNullable::False => &proc_macro2_token_stream_new,
        };
        let generated = quote::quote! {
            #postgresql_type_not_null_token_stream
            #maybe_postgresql_type_nullable_token_stream
        };
        // if ident == "" {
        //       macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //           "PostgresqlTypeTokens",
        //           &generated,
        //       );
        // }
        generated
    };
    // let postgresql_type_array = PostgresqlType::into_array().map(|element|generate_postgresql_type_token_stream(element));

    let std_primitive_i16_as_postgresql_int2_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI16AsPostgresqlInt2);
    // let std_primitive_i32_as_postgresql_int4_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI32AsPostgresqlInt4);
    // let std_primitive_i64_as_postgresql_int8_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI64AsPostgresqlInt8);
    // let std_primitive_f32_as_postgresql_float4_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4);
    // let std_primitive_f64_as_postgresql_float8_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8);
    // let std_primitive_i16_as_postgresql_small_serial_initialized_by_postgresql_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql);
    // let std_primitive_i32_as_postgresql_serial_initialized_by_postgresql_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql);
    let std_primitive_i64_as_postgresql_big_serial_initialized_by_postgresql_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql);
    // let sqlx_postgres_types_pg_money_as_postgresql_money_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney);
    // let sqlx_types_decimal_as_postgresql_numeric_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric);
    // let sqlx_types_big_decimal_as_postgresql_numeric_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric);
    // let std_primitive_bool_as_postgresql_bool_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveBoolAsPostgresqlBool);
    // let std_string_string_as_postgresql_char_n_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdStringStringAsPostgresqlCharN);
    // let std_string_string_as_postgresql_varchar_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdStringStringAsPostgresqlVarchar);
    // let std_string_string_as_postgresql_text_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdStringStringAsPostgresqlText);
    // let std_vec_vec_std_primitive_u8_as_postgresql_bytea_token_stream = generate_postgresql_type_token_stream(PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea);
    // let sqlx_types_chrono_naive_time_as_postgresql_time_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime);
    // let sqlx_types_time_time_as_postgresql_time_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime);
    // let sqlx_postgres_types_pg_interval_as_postgresql_interval_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval);
    // let sqlx_types_time_date_as_postgresql_date_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate);
    // let sqlx_types_chrono_naive_date_as_postgresql_date_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate);
    // let sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp);
    // let sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp);
    // let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz);
    // let sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz);
    // let sqlx_types_uuid_uuid_as_postgresql_uuid_v4_initialized_by_postgresql_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql);
    // let sqlx_types_uuid_uuid_as_postgresql_uuid_initialized_by_client_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient);
    // let sqlx_types_ipnetwork_ip_network_as_postgresql_inet_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet);
    // let sqlx_types_ipnetwork_ip_network_as_postgresql_cidr_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr);
    // let sqlx_types_mac_address_mac_address_as_postgresql_mac_addr_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr);
    // let sqlx_types_bit_vec_as_postgresql_bit_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesBitVecAsPostgresqlBit);
    // let sqlx_types_bit_vec_as_postgresql_varbit_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit);
    // let sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range);
    // let sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range);
    // let sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange);
    // let sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange);
    // let sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange);
    // let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange);
    // let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_range_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange);
    // let sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_range_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange);
    // let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_range_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange);
    // let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_range_token_stream = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange);

    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlTypeTokens",
    //     &std_primitive_i16_as_postgresql_int2_token_stream,
    // );

    let generated = quote::quote! {
        // #(#postgresql_type_array)*

        #std_primitive_i16_as_postgresql_int2_token_stream
        // #std_primitive_i32_as_postgresql_int4_token_stream
        // #std_primitive_i64_as_postgresql_int8_token_stream
        // #std_primitive_f32_as_postgresql_float4_token_stream
        // #std_primitive_f64_as_postgresql_float8_token_stream
        // #std_primitive_i16_as_postgresql_small_serial_initialized_by_postgresql_token_stream
        // #std_primitive_i32_as_postgresql_serial_initialized_by_postgresql_token_stream
        #std_primitive_i64_as_postgresql_big_serial_initialized_by_postgresql_token_stream
        // #sqlx_postgres_types_pg_money_as_postgresql_money_token_stream
        // #sqlx_types_decimal_as_postgresql_numeric_token_stream
        // #sqlx_types_big_decimal_as_postgresql_numeric_token_stream
        // #std_primitive_bool_as_postgresql_bool_token_stream
        // #std_string_string_as_postgresql_char_n_token_stream
        // #std_string_string_as_postgresql_varchar_token_stream
        // #std_string_string_as_postgresql_text_token_stream
        // #std_vec_vec_std_primitive_u8_as_postgresql_bytea_token_stream
        // #sqlx_types_chrono_naive_time_as_postgresql_time_token_stream
        // #sqlx_types_time_time_as_postgresql_time_token_stream
        // #sqlx_postgres_types_pg_interval_as_postgresql_interval_token_stream
        // #sqlx_types_time_date_as_postgresql_date_token_stream
        // #sqlx_types_chrono_naive_date_as_postgresql_date_token_stream
        // #sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_token_stream
        // #sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream
        // #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_token_stream
        // #sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_token_stream
        // #sqlx_types_uuid_uuid_as_postgresql_uuid_v4_initialized_by_postgresql_token_stream
        // #sqlx_types_uuid_uuid_as_postgresql_uuid_initialized_by_client_token_stream
        // #sqlx_types_ipnetwork_ip_network_as_postgresql_inet_token_stream
        // #sqlx_types_ipnetwork_ip_network_as_postgresql_cidr_token_stream
        // #sqlx_types_mac_address_mac_address_as_postgresql_mac_addr_token_stream
        // #sqlx_types_bit_vec_as_postgresql_bit_token_stream
        // #sqlx_types_bit_vec_as_postgresql_varbit_token_stream
        // #sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range_token_stream
        // #sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range_token_stream
        // #sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range_token_stream
        // #sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range_token_stream
        // #sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range_token_stream
        // #sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range_token_stream
        // #sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_range_token_stream
        // #sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_range_token_stream
        // #sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_range_token_stream
        // #sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_range_token_stream
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlTypeTokens",
    //     &generated,
    // );
    generated.into()
}
