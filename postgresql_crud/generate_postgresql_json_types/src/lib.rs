#[proc_macro]
pub fn generate_postgresql_json_types(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    fn generate_postgresql_json_type_handle_token_stream(postgresql_json_type_variant: &postgresql_crud_macros_common::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &postgresql_json_type_variant.postgresql_json_type_handle;
        let postgresql_json_type_pattern = &postgresql_json_type_variant.postgresql_json_type_pattern;
        let postgresql_json_type_ident_wrapper = postgresql_json_type_variant.postgresql_json_type_ident_wrapper();

        let ident: &dyn naming::StdFmtDisplayPlusQuoteToTokens = &postgresql_json_type_ident_wrapper;
        let field_type = &postgresql_json_type_variant.field_type();

        let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();

        let ident_token_stream = {
            let schemars_json_schema_token_stream = quote::quote! {schemars::JsonSchema,};
            let maybe_derive_schemars_json_schema_token_stream: &dyn quote::ToTokens = {
                match (&postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
                    (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath) => match &postgresql_json_type_handle {
                        postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI8 => &proc_macro2_token_stream_new,
                        postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI16 => &proc_macro2_token_stream_new,
                        postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI32
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
                pub struct #ident(pub #field_type);
            }
        };
        let maybe_impl_schemars_json_schema_for_ident_token_stream: &dyn quote::ToTokens = {
            struct SchemaObjectTokenStream<'a> {
                metadata: &'a dyn quote::ToTokens,
                instance_type: &'a dyn quote::ToTokens,
                format: &'a dyn quote::ToTokens,
                enum_values: &'a dyn quote::ToTokens,
                const_value: &'a dyn quote::ToTokens,
                subschemas: &'a dyn quote::ToTokens,
                number: &'a dyn quote::ToTokens,
                string: &'a dyn quote::ToTokens,
                array: &'a dyn quote::ToTokens,
                object: &'a dyn quote::ToTokens,
                reference: &'a dyn quote::ToTokens,
                extensions: &'a dyn quote::ToTokens,
            }
            let none_upper_camel_case = naming::NoneUpperCamelCase;
            let schema_name_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&quote::quote!{#postgresql_json_type_handle}.to_string());

            let metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream = quote::quote!{
                Some(Box::new(schemars::schema::Metadata {
                    id: None,
                    title: Some(#schema_name_format_handle_token_stream.to_owned()),
                    description: None,
                    default: None,
                    deprecated: false,
                    read_only: false,
                    write_only: false,
                    examples: std::vec::Vec::default(),
                }))
            };
            let extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream = quote::quote!{schemars::Map::default()};

            let generate_impl_schemars_json_schema_for_ident_token_stream = |schema_object_token_stream: &SchemaObjectTokenStream|{
                let schema_id_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("postgersql_crud::postgersql_json_type::{}", &quote::quote!{#postgresql_json_type_handle}));
                let metadata_token_stream = &schema_object_token_stream.metadata;
                let instance_type_token_stream = &schema_object_token_stream.instance_type;
                let format_token_stream = &schema_object_token_stream.format;
                let enum_values_token_stream = &schema_object_token_stream.enum_values;
                let const_value_token_stream = &schema_object_token_stream.const_value;
                let subschemas_token_stream = &schema_object_token_stream.subschemas;
                let number_token_stream = &schema_object_token_stream.number;
                let string_token_stream = &schema_object_token_stream.string;
                let array_token_stream = &schema_object_token_stream.array;
                let object_token_stream = &schema_object_token_stream.object;
                let reference_token_stream = &schema_object_token_stream.reference;
                let extensions_token_stream = &schema_object_token_stream.extensions;
                quote::quote! {
                    impl schemars::JsonSchema for #ident {
                        fn schema_name() -> std::string::String {
                            #schema_name_format_handle_token_stream.to_owned()
                        }
                        fn schema_id() -> std::borrow::Cow<'static, str> {
                            std::borrow::Cow::Borrowed(#schema_id_format_handle_token_stream)
                        }
                        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
                            {
                                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                                    metadata: #metadata_token_stream,
                                    instance_type: #instance_type_token_stream,
                                    format: #format_token_stream,
                                    enum_values: #enum_values_token_stream,
                                    const_value: #const_value_token_stream,
                                    subschemas: #subschemas_token_stream,
                                    number: #number_token_stream,
                                    string: #string_token_stream,
                                    array: #array_token_stream,
                                    object: #object_token_stream,
                                    reference: #reference_token_stream,
                                    extensions: #extensions_token_stream,
                                })
                            }
                        }
                    }
                }
            };
            match (&postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
                (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath) => match &postgresql_json_type_handle {
                    postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI8 => &generate_impl_schemars_json_schema_for_ident_token_stream(
                        &SchemaObjectTokenStream {
                            metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream,
                            instance_type: &quote::quote!{Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::Number)))},
                            format: &none_upper_camel_case,
                            enum_values: &none_upper_camel_case,
                            const_value: &none_upper_camel_case,
                            subschemas: &none_upper_camel_case,
                            number: &quote::quote!{Some(Box::new(schemars::schema::NumberValidation {
                                multiple_of: None,
                                maximum: Some(127.0),
                                exclusive_maximum: None,
                                minimum: Some(-128.0),
                                exclusive_minimum: None,
                            }))},
                            string: &none_upper_camel_case,
                            array: &none_upper_camel_case,
                            object: &none_upper_camel_case,
                            reference: &none_upper_camel_case,
                            extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream,
                        },
                    ),
                    postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI16 => &generate_impl_schemars_json_schema_for_ident_token_stream(
                        &SchemaObjectTokenStream {
                            metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream,
                            instance_type: &quote::quote!{Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::Number)))},
                            format: &none_upper_camel_case,
                            enum_values: &none_upper_camel_case,
                            const_value: &none_upper_camel_case,
                            subschemas: &none_upper_camel_case,
                            number: &quote::quote!{Some(Box::new(schemars::schema::NumberValidation {
                                multiple_of: None,
                                maximum: Some(32767),
                                exclusive_maximum: None,
                                minimum: Some(-32768),
                                exclusive_minimum: None,
                            }))},
                            string: &none_upper_camel_case,
                            array: &none_upper_camel_case,
                            object: &none_upper_camel_case,
                            reference: &none_upper_camel_case,
                            extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream,
                        },
                    ),
                    postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI32
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI64
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU8
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU16
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU32
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveU64
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveF32
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveF64
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveBool
                    | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdStringString => &proc_macro2_token_stream_new,
                    postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid => &generate_impl_schemars_json_schema_for_ident_token_stream(
                        &SchemaObjectTokenStream {
                            metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream,
                            instance_type: &quote::quote!{Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::String)))},
                            format: &none_upper_camel_case,
                            enum_values: &none_upper_camel_case,
                            const_value: &none_upper_camel_case,
                            subschemas: &none_upper_camel_case,
                            number: &none_upper_camel_case,
                            string: &quote::quote!{Some(Box::new(schemars::schema::StringValidation {
                                max_length: Some(36),
                                min_length: Some(36),
                                pattern: None,
                            }))},
                            array: &none_upper_camel_case,
                            object: &none_upper_camel_case,
                            reference: &none_upper_camel_case,
                            extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream,
                        },
                    )
                },
                (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath)
                | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
                | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
                | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath)
                | (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => &proc_macro2_token_stream_new,
            }
        };
        let maybe_impl_is_empty_for_ident_token_stream: &dyn quote::ToTokens = match (&postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
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
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveBool => &proc_macro2_token_stream_new,
                postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdStringString => &postgresql_crud_macros_common::generate_impl_crate_is_empty_for_ident_token_stream(&postgresql_json_type_ident_wrapper),
                postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid => &postgresql_crud_macros_common::generate_impl_crate_is_empty_for_ident_token_stream(&postgresql_json_type_ident_wrapper),
            },
            (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath) => match &postgresql_json_type_handle {
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
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdStringString
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid => &proc_macro2_token_stream_new,
            },
            (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => match &postgresql_json_type_handle {
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
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdStringString
                | postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid => &proc_macro2_token_stream_new,
            },
            (postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True, postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath)
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

        let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_token_stream = postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident, &{
            let content_token_stream = postgresql_json_type_variant.initialization_token_stream();
            quote::quote! {
                Self(#content_token_stream)
            }
        });
        let impl_error_occurence_lib_to_std_string_string_for_ident_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self:#?}")});

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
        let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream =
            postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_upper_camel_case, &{
                let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
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
                                pagination: #crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            }
                        }
                    }
                    postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => todo!(),
                }
            });
        let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
        let ident_read_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_read_upper_camel_case, &ident);

        let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);

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
            let is_vec_element_type = postgresql_json_type_variant.is_vec_element_type();
            let common_postgresql_json_type_filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = vec![&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal];
            let common_postgresql_json_type_vec_filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = {
                let mut vec: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = common_postgresql_json_type_filters_variants.clone();
                vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthMoreThan);
                if is_vec_element_type {
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::PositionEqual);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsAllElementsOfArray);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::OverlapsWithArray);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual);
                }
                vec
            };
            //this is for not nullable\not optionable filters like GreaterThan, Regular expression, etc.
            let postgresql_json_type_ident_wrapper_relevant_only_for_not_null = postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
                postgresql_json_type_handle: postgresql_json_type_variant.postgresql_json_type_handle.clone(),
                postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
                    postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False,
                    postgresql_json_type_pattern_type: postgresql_json_type_variant.postgresql_json_type_pattern.postgresql_json_type_pattern_type.clone(),
                },
            }
            .postgresql_json_type_ident_wrapper();
            let generate_where_element_variants_types_generic_token_stream =
                |is_relevant_only_for_not_null: std::primitive::bool| -> &dyn naming::StdFmtDisplayPlusQuoteToTokens { if is_relevant_only_for_not_null { &postgresql_json_type_ident_wrapper_relevant_only_for_not_null } else { &postgresql_json_type_ident_wrapper } };
            let postgresql_json_type_where_element_number_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                &{
                    let mut vec = common_postgresql_json_type_filters_variants.clone();
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In);
                    vec
                },
                &generate_where_element_variants_types_generic_token_stream,
                &ident,
                &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                &postgresql_crud_macros_common::IsQueryBindMutable::False,
            );
            let postgresql_json_type_where_element_bool_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                &common_postgresql_json_type_filters_variants,
                &generate_where_element_variants_types_generic_token_stream,
                &ident,
                &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                &postgresql_crud_macros_common::IsQueryBindMutable::False,
            );
            let postgresql_json_type_where_element_string_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                &{
                    let mut vec = common_postgresql_json_type_filters_variants.clone();
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::CaseSensitiveRegularExpression);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::CaseInsensitiveRegularExpression);
                    vec
                },
                &generate_where_element_variants_types_generic_token_stream,
                &ident,
                &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                &postgresql_crud_macros_common::IsQueryBindMutable::False,
            );

            let postgresql_json_type_where_element_vec_number_token_stream = {
                let mut filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = common_postgresql_json_type_vec_filters_variants.clone();
                if is_vec_element_type {
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::PositionGreaterThan);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementGreaterThan);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan);
                }
                postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &generate_where_element_variants_types_generic_token_stream,
                    &ident,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                    &postgresql_crud_macros_common::IsQueryBindMutable::False,
                )
            };
            let postgresql_json_type_where_element_vec_bool_token_stream = {
                let filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = common_postgresql_json_type_vec_filters_variants.clone();
                postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &generate_where_element_variants_types_generic_token_stream,
                    &ident,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                    &postgresql_crud_macros_common::IsQueryBindMutable::False,
                )
            };
            let postgresql_json_type_where_element_vec_string_token_stream = {
                let mut filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = common_postgresql_json_type_vec_filters_variants.clone();
                if is_vec_element_type {
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::PositionCaseSensitiveRegularExpression);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::PositionCaseInsensitiveRegularExpression);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementCaseSensitiveRegularExpression);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementCaseInsensitiveRegularExpression);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsCaseSensitiveRegularExpression);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsCaseInsensitiveRegularExpression);
                }
                postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &generate_where_element_variants_types_generic_token_stream,
                    &ident,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                    &postgresql_crud_macros_common::IsQueryBindMutable::False,
                )
            };
            match &postgresql_json_type_variant.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => postgresql_json_type_where_element_number_token_stream,
                    PostgresqlJsonTypeSpecific::Bool => postgresql_json_type_where_element_bool_token_stream,
                    PostgresqlJsonTypeSpecific::String => postgresql_json_type_where_element_string_token_stream,
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => postgresql_json_type_where_element_vec_number_token_stream,
                    PostgresqlJsonTypeSpecific::Bool => postgresql_json_type_where_element_vec_bool_token_stream,
                    PostgresqlJsonTypeSpecific::String => postgresql_json_type_where_element_vec_string_token_stream,
                },
                postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => postgresql_json_type_where_element_vec_number_token_stream,
                    PostgresqlJsonTypeSpecific::Bool => postgresql_json_type_where_element_vec_bool_token_stream,
                    PostgresqlJsonTypeSpecific::String => postgresql_json_type_where_element_vec_string_token_stream,
                },
            }
        };
        let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
        let ident_update_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_update_upper_camel_case, &ident);
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let impl_crate_postgresql_json_type_for_ident_token_stream = postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
            &quote::quote! {crate::},
            &ident,
            &ident_create_upper_camel_case,
            &{
                quote::quote! {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!("${increment}"))
                        }
                        None => Err(crate::QueryPartErrorNamed::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!()
                        }),
                    }
                }
            },
            &postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
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
            &{
                let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{jsonb_set_path}}}}}}',${{increment}})"));
                quote::quote! {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!(#format_handle_token_stream))
                        }
                        None => Err(crate::QueryPartErrorNamed::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                    }
                }
            },
            &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
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
            #maybe_impl_is_empty_for_ident_token_stream
            #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_token_stream

            #ident_create_alias_token_stream
            #ident_select_token_stream
            #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream
            #ident_read_alias_token_stream
            #postgresql_json_type_ident_where_element_token_stream
            // #postgresql_json_type_ident_where_element_second_dimension_token_stream
            #ident_update_alias_token_stream
            #impl_crate_postgresql_json_type_for_ident_token_stream
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
        postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdPrimitiveI8,
            postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False,
                postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath,
            },
        },
        postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False,
                postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath,
            },
        },
        // postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdStringString,
        //     postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False,
        //         postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath,
        //     },
        // },
        // postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::StdStringString,
        //     postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::True,
        //         postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::FullTypePath,
        //     },
        // },
        // postgresql_crud_macros_common::PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: postgresql_crud_macros_common::PostgresqlJsonTypeHandle::UuidUuid,
        //     postgresql_json_type_pattern: postgresql_crud_macros_common::PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: postgresql_crud_macros_common::PostgresqlJsonTypePatternIsOptional::False,
        //         postgresql_json_type_pattern_type: postgresql_crud_macros_common::PostgresqlJsonTypePatternType::StdVecVecFullTypePath,
        //     },
        // },
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
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file("GeneratePostgresqlJsonTypes", &generated);
    generated.into()
}
