#[proc_macro]
pub fn generate_postgresql_json_types(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    fn generate_postgresql_json_type_handle_token_stream(postgresql_json_type_variant: &postgresql_crud_macros_common::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
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
                    (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath) => match &postgresql_json_type_handle {
                        postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI8
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI16
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI32
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI64
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU8
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU16
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU32
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU64
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveF32
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveF64
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveBool
                        | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdStringString => &schemars_json_schema_token_stream,
                        postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid => &proc_macro2_token_stream_new,
                    },
                    (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath)
                    | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
                    | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
                    | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath)
                    | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => &schemars_json_schema_token_stream,
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
            (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath) => match &postgresql_json_type_handle {
                postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI8
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI16
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI32
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI64
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU8
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU16
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU32
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU64
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveF32
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveF64
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveBool
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdStringString => &proc_macro2_token_stream_new,
                postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid => &{
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
            (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath)
            | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
            | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
            | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath)
            | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => &proc_macro2_token_stream_new,
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

        let impl_crate_generate_postgresql_json_type_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_token_stream =
            postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident, &{
                let content_token_stream = postgresql_json_type_variant.initialization_token_stream();
                quote::quote! {
                    Self(#content_token_stream)
                }
            });
        let impl_error_occurence_lib_to_std_string_string_for_ident_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&ident, &quote::quote! {format!("{self:#?}")});

        let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
        let ident_create_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_create_upper_camel_case, &ident);
        let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
        let ident_select_token_stream = {
            let content_token_stream = match &postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath => quote::quote! {{}},
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath => quote::quote! {{ pagination: crate::pagination::Pagination }},
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => quote::quote! {{ pagination: crate::pagination::Pagination }}, //todo another pagination?
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
                pub struct #ident_select_upper_camel_case #content_token_stream
            }
        };
        let impl_crate_generate_postgresql_json_type_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream =
            postgresql_crud_macros_common::generate_impl_crate_generate_postgresql_json_type_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_upper_camel_case, &{
                let crate_generate_postgresql_json_type_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream =
                    token_patterns::CrateGeneratePostgresqlJsonTypeDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
                match &postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                    postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath => {
                        let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                        quote::quote! {
                            #core_default_default_default
                        }
                    }
                    postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath => {
                        quote::quote! {
                            Self {
                                pagination: #crate_generate_postgresql_json_type_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            }
                        }
                    }
                    postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => {
                        quote::quote! {
                            Self {
                                pagination: #crate_generate_postgresql_json_type_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            }
                        }
                    }
                }
            });
        let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
        let ident_read_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_read_upper_camel_case, &ident);

        let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
        
        enum MaybePostgresqlJsonTypeIdentWhereElementFilter<'a> {
            Some { where_operator_name: &'a dyn postgresql_crud_macros_common::WhereOperatorName, token_stream: proc_macro2::TokenStream },
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
        impl std::convert::From<&postgresql_crud_macros_common::PostgresqlJsonTypeHandle> for PostgresqlJsonTypeSpecific {
            fn from(value: &postgresql_crud_macros_common::PostgresqlJsonTypeHandle) -> Self {
                match value {
                    postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI8
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI16
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI32
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI64
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU8
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU16
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU32
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU64
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveF32
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveF64 => Self::Number,
                    postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveBool => Self::Bool,
                    postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdStringString | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid => Self::String,
                }
            }
        }
        let postgresql_json_type_specific = PostgresqlJsonTypeSpecific::from(&postgresql_json_type_variant.postgresql_json_type_handle);
        let postgresql_json_type_ident_where_element_token_stream = {
            let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);

            let equal = postgresql_crud_macros_common::Equal;
            let postgresql_json_type_ident_where_element_equal_token_stream = equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);

            let common_postgresql_json_type_filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::WhereOperatorName> = vec![&equal];
            let common_postgresql_json_type_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = vec![postgresql_json_type_ident_where_element_equal_token_stream];

            let length_equal = postgresql_crud_macros_common::LengthEqual;
            let postgresql_json_type_ident_where_element_length_equal_token_stream = length_equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);
            let length_more_than = postgresql_crud_macros_common::LengthMoreThan;
            let postgresql_json_type_ident_where_element_length_more_than_token_stream = length_more_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);

            let position_equal = postgresql_crud_macros_common::PositionEqual;
            let position_greater_than = postgresql_crud_macros_common::PositionGreaterThan;
            let position_case_sensitive_regular_expression = postgresql_crud_macros_common::PositionCaseSensitiveRegularExpression;
            let position_case_insensitive_regular_expression = postgresql_crud_macros_common::PositionCaseInsensitiveRegularExpression;
            let contains_all_elements_of_array = postgresql_crud_macros_common::ContainsAllElementsOfArray;
            // let contained_in_array = postgresql_crud_macros_common::ContainedInArray;
            let overlaps_with_array = postgresql_crud_macros_common::OverlapsWithArray;
            let all_elements_equal = postgresql_crud_macros_common::AllElementsEqual;
            let contains_element_greater_than = postgresql_crud_macros_common::ContainsElementGreaterThan;
            let all_elements_greater_than = postgresql_crud_macros_common::AllElementsGreaterThan;
            let contains_element_case_sensitive_regular_expression = postgresql_crud_macros_common::ContainsElementCaseSensitiveRegularExpression;
            let contains_element_case_insensitive_regular_expression = postgresql_crud_macros_common::ContainsElementCaseInsensitiveRegularExpression;
            let all_elements_case_sensitive_regular_expression = postgresql_crud_macros_common::AllElementsCaseSensitiveRegularExpression;
            let all_elements_case_insensitive_regular_expression = postgresql_crud_macros_common::AllElementsCaseInsensitiveRegularExpression;

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

            let mut common_postgresql_json_type_vec_filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::WhereOperatorName> = {
                let mut vec: std::vec::Vec<&dyn postgresql_crud_macros_common::WhereOperatorName> = common_postgresql_json_type_filters_variants.clone();
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
                let greater_than = postgresql_crud_macros_common::GreaterThan;
                let postgresql_json_type_ident_where_element_greater_than_token_stream = greater_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);
                let between = postgresql_crud_macros_common::Between;
                let postgresql_json_type_ident_where_element_between_token_stream = between.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&postgresql_crud_macros_common::BetweenTryNewErrorType::StartMoreOrEqualToEnd, postgresql_json_type_variant);
                let in_handle = postgresql_crud_macros_common::In;
                let postgresql_json_type_ident_where_element_in_token_stream = in_handle.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);
                //todo write wrapper around it with reuse parameters
                let postgresql_json_type_ident_where_element_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &{
                        let mut vec = common_postgresql_json_type_filters_variants.clone();
                        vec.push(&greater_than);
                        vec.push(&between);
                        vec.push(&in_handle);
                        vec
                    },
                    &ident_where_element_upper_camel_case,
                    &ident_where_element_upper_camel_case,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
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
                let postgresql_json_type_ident_where_element_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &common_postgresql_json_type_filters_variants,
                    &ident_where_element_upper_camel_case,
                    &ident_where_element_upper_camel_case,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
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
                let case_sensitive_regular_expression = postgresql_crud_macros_common::CaseSensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&ident);
                let case_insensitive_regular_expression = postgresql_crud_macros_common::CaseInsensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&ident);

                let postgresql_json_type_ident_where_element_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &{
                        let mut vec = common_postgresql_json_type_filters_variants.clone();
                        vec.push(&case_sensitive_regular_expression);
                        vec.push(&case_insensitive_regular_expression);
                        vec
                    },
                    &ident_where_element_upper_camel_case,
                    &ident_where_element_upper_camel_case,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
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
                let mut filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
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
                let postgresql_json_type_ident_where_element_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &ident_where_element_upper_camel_case,
                    &ident_where_element_upper_camel_case,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True
                );
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
                let filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
                let filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_vec_filters_token_stream.clone();
                let postgresql_json_type_ident_where_element_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &ident_where_element_upper_camel_case,
                    &ident_where_element_upper_camel_case,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True
                );
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
                let mut filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
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
                let postgresql_json_type_ident_where_element_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &ident_where_element_upper_camel_case,
                    &ident_where_element_upper_camel_case,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True
                );
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
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => generate_postgresql_json_type_where_element_number_token_stream(),
                    PostgresqlJsonTypeSpecific::Bool => generate_postgresql_json_type_where_element_bool_token_stream(),
                    PostgresqlJsonTypeSpecific::String => generate_postgresql_json_type_where_element_string_token_stream(),
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => generate_postgresql_json_type_where_element_vec_number_token_stream(),
                    PostgresqlJsonTypeSpecific::Bool => generate_postgresql_json_type_where_element_vec_bool_token_stream(),
                    PostgresqlJsonTypeSpecific::String => generate_postgresql_json_type_where_element_vec_string_token_stream(),
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => generate_postgresql_json_type_where_element_vec_number_token_stream(),
                    PostgresqlJsonTypeSpecific::Bool => generate_postgresql_json_type_where_element_vec_bool_token_stream(),
                    PostgresqlJsonTypeSpecific::String => generate_postgresql_json_type_where_element_vec_string_token_stream(),
                },
            }
        };
        //
        let postgresql_json_type_ident_where_element_second_dimension_token_stream = {
            let ident_where_element_second_dimension_upper_camel_case = naming::parameter::SelfWhereElementSecondDimensionUpperCamelCase::from_tokens(&ident);

            let equal_second_dimension = postgresql_crud_macros_common::EqualSecondDimension;
            let postgresql_json_type_ident_where_element_equal_second_dimension_token_stream = equal_second_dimension.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(postgresql_json_type_variant);

            let common_postgresql_json_type_filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::WhereOperatorName> = vec![&equal_second_dimension];
            let common_postgresql_json_type_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = vec![postgresql_json_type_ident_where_element_equal_second_dimension_token_stream];

            // let length_equal_second_dimension = postgresql_crud_macros_common::LengthEqualSecondDimension;
            // let postgresql_json_type_ident_where_element_length_equal_token_stream = length_equal_second_dimension.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
            //     &postgresql_json_type_variant,
            // );

            let common_postgresql_json_type_vec_filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::WhereOperatorName> = common_postgresql_json_type_filters_variants.clone();
            let common_postgresql_json_type_vec_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_filters_token_stream.clone();

            let generate_where_element_vec_string_second_dimension_token_stream = || {
                let filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
                let filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_vec_filters_token_stream.clone();
                let ident_where_element_second_dimension_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &ident_where_element_second_dimension_upper_camel_case,
                    &ident_where_element_upper_camel_case,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote! {
                    #(#filters_token_stream)*
                    #ident_where_element_second_dimension_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            match &postgresql_json_type_variant.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::Bool => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::String => &proc_macro2_token_stream_new,
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::Bool => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::String => &generate_where_element_vec_string_second_dimension_token_stream(),
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::Bool => &proc_macro2_token_stream_new,
                    PostgresqlJsonTypeSpecific::String => &proc_macro2_token_stream_new,
                },
            }
        };
        // println!("{}", quote::quote!{#postgresql_json_type_ident_where_element_second_dimension_token_stream});
        //
        let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
        let ident_update_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_update_upper_camel_case, &ident);
        let ident_update_error_named_upper_camel_case = naming::parameter::SelfUpdateErrorNamedUpperCamelCase::from_tokens(&ident);
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let postgresql_json_type_ident_option_to_update_try_generate_bind_increments_error_named_token_stream = {
            quote::quote! {
                #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #ident_update_error_named_upper_camel_case {
                    #checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
                }
            }
        };
        let impl_crate_generate_postgresql_json_type_postgresql_json_type_for_ident_token_stream = postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
            &quote::quote! {crate::postgresql_json_type::postgresql_json_type_trait::},
            &ident,
            &ident_create_upper_camel_case,
            &{
                //todo reuse
                let crate_postgresql_json_type_postgresql_json_type_trait_create_query_part_error_named_token_stream = quote::quote! {
                    crate::postgresql_json_type::postgresql_json_type_trait::CreateQueryPartErrorNamed
                };
                quote::quote! {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!("${increment}"))
                        }
                        None => Err(#crate_postgresql_json_type_postgresql_json_type_trait_create_query_part_error_named_token_stream::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!()
                        }),
                    }
                }
            },
            &{
                let value_snake_case = naming::ValueSnakeCase;
                quote::quote! {
                    query = query.bind(sqlx::types::Json(#value_snake_case.0));
                    query
                }
            },
            &ident_select_upper_camel_case,
            &ident_read_upper_camel_case,
            &{
                let value_snake_case = naming::ValueSnakeCase;
                let postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream = |format_handle_token_stream: &dyn quote::ToTokens| {
                    let pagination_start_end_initialization_token_stream = macros_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream(&value_snake_case);
                    quote::quote! {
                        #pagination_start_end_initialization_token_stream
                        format!(#format_handle_token_stream)
                    }
                };
                let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;

                match &postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                    postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath => {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('{{field_ident}}', jsonb_build_object('value', {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}'))"));
                        quote::quote! {
                            format!(#format_handle_token_stream)
                        }
                    }
                    postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_quotes::double_quotes_token_stream(&format!(
                        "jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))"
                    ))),
                    // postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdOptionOptionFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                    //     &generate_quotes::double_quotes_token_stream(
                    //         &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value', case when jsonb_typeof({{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}') = 'array' then (select jsonb_agg(value) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}}) else null end))")
                    //     )
                    // ),
                    postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_quotes::double_quotes_token_stream(&format!(
                        "jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))"
                    ))),
                }
            },
            &ident_where_element_upper_camel_case,
            &ident_update_upper_camel_case,
            &ident_update_error_named_upper_camel_case,
            &{
                let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{jsonb_set_path}}}}}}',${{increment}})"));
                let update_query_part_error_named_upper_camel_case = naming::UpdateQueryPartErrorNamedUpperCamelCase;
                quote::quote! {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!(#format_handle_token_stream))
                        }
                        None => Err(Self::#update_query_part_error_named_upper_camel_case::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                    }
                }
            },
            &{
                let value_snake_case = naming::ValueSnakeCase;
                quote::quote! {
                    query = query.bind(sqlx::types::Json(#value_snake_case.0));
                    query
                }
            },
        );
        //todo maybe impl Encode instead of just wrap into sqlx::types::Json
        let generated = quote::quote! {
            #ident_token_stream
            #maybe_impl_schemars_json_schema_for_ident_token_stream
            #impl_crate_generate_postgresql_json_type_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_token_stream

            #ident_create_alias_token_stream
            #ident_select_token_stream
            #impl_crate_generate_postgresql_json_type_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream
            #ident_read_alias_token_stream
            #postgresql_json_type_ident_where_element_token_stream
            #postgresql_json_type_ident_where_element_second_dimension_token_stream
            #ident_update_alias_token_stream
            #postgresql_json_type_ident_option_to_update_try_generate_bind_increments_error_named_token_stream
            #impl_crate_generate_postgresql_json_type_postgresql_json_type_for_ident_token_stream
        };
        // println!("{}", quote::quote!{#ident});
        // if quote::quote!{#ident}.to_string() == "StdVecVecStdVecVecUuidUuid" {
        //    //  println!("{generated}");
        //    //  println!("-------");
            // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            //     "PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeTime",
            //     &generated,
            // );
        //     generated = quote::quote!{}
        // }
        generated
    }
    // postgresql_crud_macros_common::PostgresqlJsonTypeVariant::all_variants()
    let variants_token_stream = [
        // postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI8,
        //     postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False,
        //         postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath,
        //     },
        // },
        postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False,
                postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath,
            },
        },
        postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True,
                postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath,
            },
        },
        postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False,
                postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath,
            },
        },
        // postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid,
        //     postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True,
        //         postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath,
        //     },
        // },
        // postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid,
        //     postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False,
        //         postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath,
        //     },
        // },
        // postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid,
        //     postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True,
        //         postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath,
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