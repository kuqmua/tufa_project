#[proc_macro]
pub fn generate_postgresql_json_types(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();

    #[derive(Debug, strum_macros::Display)]
    enum RustTypeName {
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
    impl std::convert::From<&PostgresqlJsonType> for RustTypeName {
        fn from(value: &PostgresqlJsonType) -> Self {
            match &value {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => Self::StdPrimitiveI8,
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => Self::StdPrimitiveI16,
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => Self::StdPrimitiveI32,
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => Self::StdPrimitiveI64,
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => Self::StdPrimitiveU8,
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => Self::StdPrimitiveU16,
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => Self::StdPrimitiveU32,
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => Self::StdPrimitiveU64,
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => Self::StdPrimitiveF32,
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => Self::StdPrimitiveF64,
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::StdPrimitiveBool,
                PostgresqlJsonType::StdStringStringAsJsonbString => Self::StdStringString,
                PostgresqlJsonType::UuidUuidAsJsonbString => Self::UuidUuid,
            }
        }
    }
    #[derive(Debug)]
    enum PostgresqlJsonTypeName {
        Number,
        Boolean,
        String,
    }
    impl std::fmt::Display for PostgresqlJsonTypeName {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                formatter,
                "{}",
                naming::parameter::JsonbSelfUpperCamelCase::from_display(match &self {
                    Self::Number => &naming::NumberUpperCamelCase,
                    Self::Boolean => &naming::BooleanUpperCamelCase,
                    Self::String => &naming::StringUpperCamelCase,
                })
            )
        }
    }
    impl std::convert::From<&PostgresqlJsonType> for PostgresqlJsonTypeName {
        fn from(value: &PostgresqlJsonType) -> Self {
            match &value {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::Boolean,
                PostgresqlJsonType::StdStringStringAsJsonbString => Self::String,
                PostgresqlJsonType::UuidUuidAsJsonbString => Self::String,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    enum PostgresqlJsonType {
        StdPrimitiveI8AsJsonbNumber,
        StdPrimitiveI16AsJsonbNumber,
        StdPrimitiveI32AsJsonbNumber,
        StdPrimitiveI64AsJsonbNumber,
        StdPrimitiveU8AsJsonbNumber,
        StdPrimitiveU16AsJsonbNumber,
        StdPrimitiveU32AsJsonbNumber,
        StdPrimitiveU64AsJsonbNumber,
        StdPrimitiveF32AsJsonbNumber,
        StdPrimitiveF64AsJsonbNumber,
        StdPrimitiveBoolAsJsonbBoolean,
        StdStringStringAsJsonbString,
        UuidUuidAsJsonbString,
    }
    impl quote::ToTokens for PostgresqlJsonType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string().parse::<proc_macro2::TokenStream>().expect("error eb6cafe0-ad0d-4108-8b0e-c062b155efbb").to_tokens(tokens)
        }
    }
    #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    enum PostgresqlJsonTypePattern {
        Standart,
        ArrayDimension1 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension2 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension3 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension3_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension4 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension3_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension4_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
    }
    enum ArrayDimension {
        ArrayDimension1,
        ArrayDimension2 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension3 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension4 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension3_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
    }
    impl ArrayDimension {
        pub fn to_usize(&self) -> std::primitive::usize {
            match &self {
                Self::ArrayDimension1 { .. } => 1,
                Self::ArrayDimension2 { .. } => 2,
                Self::ArrayDimension3 { .. } => 3,
                Self::ArrayDimension4 { .. } => 4,
            }
        }
    }
    impl std::convert::TryFrom<&PostgresqlJsonTypePattern> for ArrayDimension {
        type Error = ();
        fn try_from(value: &PostgresqlJsonTypePattern) -> Result<Self, Self::Error> {
            match &value {
                PostgresqlJsonTypePattern::Standart => Err(()),
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: _ } => Ok(Self::ArrayDimension1),
                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable: _ } => Ok(Self::ArrayDimension2 { dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable }),
                PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable: _,
                } => Ok(Self::ArrayDimension3 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                }),
                PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable: _,
                } => Ok(Self::ArrayDimension4 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable: *dimension3_not_null_or_nullable,
                }),
            }
        }
    }
    enum ArrayDimensionSelectPattern {
        ArrayDimension2 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension3 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension4 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension3_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
    }
    impl ArrayDimensionSelectPattern {
        fn to_usize(&self) -> std::primitive::usize {
            match &self {
                Self::ArrayDimension2 { .. } => 2,
                Self::ArrayDimension3 { .. } => 3,
                Self::ArrayDimension4 { .. } => 4,
            }
        }
        fn select_array(&self) -> std::vec::Vec<&postgresql_crud_macros_common::NotNullOrNullable> {
            match &self {
                Self::ArrayDimension2 { dimension1_not_null_or_nullable } => vec![&dimension1_not_null_or_nullable],
                Self::ArrayDimension3 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => vec![&dimension2_not_null_or_nullable, &dimension1_not_null_or_nullable],
                Self::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                } => vec![&dimension3_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension1_not_null_or_nullable],
            }
        }
    }
    impl std::convert::TryFrom<&ArrayDimension> for ArrayDimensionSelectPattern {
        type Error = ();
        fn try_from(value: &ArrayDimension) -> Result<Self, Self::Error> {
            match &value {
                ArrayDimension::ArrayDimension1 => Err(()),
                ArrayDimension::ArrayDimension2 { dimension1_not_null_or_nullable } => Ok(Self::ArrayDimension2 { dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable }),
                ArrayDimension::ArrayDimension3 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => Ok(Self::ArrayDimension3 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                }),
                ArrayDimension::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                } => Ok(Self::ArrayDimension4 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable: *dimension3_not_null_or_nullable,
                }),
            }
        }
    }
    #[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    struct PostgresqlJsonTypeRecord {
        postgresql_json_type: PostgresqlJsonType,
        not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        postgresql_json_type_pattern: PostgresqlJsonTypePattern,
    }
    impl PostgresqlJsonTypeRecord {
        fn all() -> std::vec::Vec<Self> {
            PostgresqlJsonType::into_array().into_iter().fold(vec![], |mut acc, postgresql_json_type| {
                PostgresqlJsonTypePattern::into_array().into_iter().for_each(|postgresql_json_type_pattern| match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => {
                        postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| {
                            acc.push(PostgresqlJsonTypeRecord {
                                postgresql_json_type: postgresql_json_type.clone(),
                                not_null_or_nullable,
                                postgresql_json_type_pattern: postgresql_json_type_pattern.clone(),
                            });
                        });
                    }
                    PostgresqlJsonTypePattern::ArrayDimension1 { .. } => {
                        postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| {
                            postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                                acc.push(PostgresqlJsonTypeRecord {
                                    postgresql_json_type: postgresql_json_type.clone(),
                                    not_null_or_nullable,
                                    postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable },
                                });
                            });
                        });
                    }
                    PostgresqlJsonTypePattern::ArrayDimension2 { .. } => {
                        postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| {
                            postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                                postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension2_not_null_or_nullable| {
                                    acc.push(PostgresqlJsonTypeRecord {
                                        postgresql_json_type: postgresql_json_type.clone(),
                                        not_null_or_nullable,
                                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable },
                                    });
                                });
                            });
                        });
                    }
                    PostgresqlJsonTypePattern::ArrayDimension3 { .. } => {
                        postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| {
                            postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                                postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension2_not_null_or_nullable| {
                                    postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension3_not_null_or_nullable| {
                                        acc.push(PostgresqlJsonTypeRecord {
                                            postgresql_json_type: postgresql_json_type.clone(),
                                            not_null_or_nullable,
                                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension3 {
                                                dimension1_not_null_or_nullable,
                                                dimension2_not_null_or_nullable,
                                                dimension3_not_null_or_nullable,
                                            },
                                        });
                                    });
                                });
                            });
                        });
                    }
                    PostgresqlJsonTypePattern::ArrayDimension4 { .. } => {
                        postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| {
                            postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                                postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension2_not_null_or_nullable| {
                                    postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension3_not_null_or_nullable| {
                                        postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension4_not_null_or_nullable| {
                                            acc.push(PostgresqlJsonTypeRecord {
                                                postgresql_json_type: postgresql_json_type.clone(),
                                                not_null_or_nullable,
                                                postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension4 {
                                                    dimension1_not_null_or_nullable,
                                                    dimension2_not_null_or_nullable,
                                                    dimension3_not_null_or_nullable,
                                                    dimension4_not_null_or_nullable,
                                                },
                                            });
                                        });
                                    });
                                });
                            });
                        });
                    }
                });
                acc
            })
        }
    }
    #[derive(Debug, serde::Deserialize)]
    enum GeneratePostgresqlJsonTypesConfig {
        All,
        Concrete(std::vec::Vec<PostgresqlJsonTypeRecord>),
    }
    let postgresql_json_type_record_vec = {
        let generate_postgresql_json_types_config = serde_json::from_str::<GeneratePostgresqlJsonTypesConfig>(&input_token_stream.to_string()).expect("failed to get Config for generate_postgresql_json_types");
        let postgresql_json_type_record_vec = match generate_postgresql_json_types_config {
            GeneratePostgresqlJsonTypesConfig::All => PostgresqlJsonTypeRecord::all(),
            GeneratePostgresqlJsonTypesConfig::Concrete(value) => value,
        };
        {
            let mut seen = std::collections::HashSet::new();
            if !postgresql_json_type_record_vec.iter().all(|element| seen.insert(element)) {
                panic!("not unique postgersql type provided: {postgresql_json_type_record_vec:#?}");
            }
        }
        postgresql_json_type_record_vec.into_iter().fold(vec![], |mut acc, postgresql_json_type_record_element| {
            use postgresql_crud_macros_common::NotNullOrNullable;
            #[derive(Clone)]
            struct PostgresqlJsonTypeRecordHandle {
                not_null_or_nullable: NotNullOrNullable,
                postgresql_json_type_pattern: PostgresqlJsonTypePattern,
            }
            fn generate_postgresql_json_type_record_handle_vec(postgresql_json_type_record_handle: PostgresqlJsonTypeRecordHandle) -> std::vec::Vec<PostgresqlJsonTypeRecordHandle> {
                let generate_vec = |current_postgresql_json_type_record_handle: PostgresqlJsonTypeRecordHandle| generate_postgresql_json_type_record_handle_vec(current_postgresql_json_type_record_handle).into_iter().chain(std::iter::once(postgresql_json_type_record_handle.clone())).collect();
                match (&postgresql_json_type_record_handle.not_null_or_nullable, &postgresql_json_type_record_handle.postgresql_json_type_pattern) {
                    (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) => vec![postgresql_json_type_record_handle],
                    (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::Standart) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: NotNullOrNullable::NotNull,
                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::Standart,
                    }),
                    (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: *dimension1_not_null_or_nullable,
                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::Standart,
                    }),
                    (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension1 { .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: NotNullOrNullable::NotNull,
                        postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.clone(),
                    }),
                    (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: *dimension1_not_null_or_nullable,
                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable },
                    }),
                    (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension2 { .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: NotNullOrNullable::NotNull,
                        postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.clone(),
                    }),
                    (
                        NotNullOrNullable::NotNull,
                        PostgresqlJsonTypePattern::ArrayDimension3 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                        },
                    ) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: *dimension1_not_null_or_nullable,
                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension2 {
                            dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                            dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                        },
                    }),
                    (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension3 { .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: NotNullOrNullable::NotNull,
                        postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.clone(),
                    }),
                    (
                        NotNullOrNullable::NotNull,
                        PostgresqlJsonTypePattern::ArrayDimension4 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                            dimension4_not_null_or_nullable,
                        },
                    ) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: *dimension1_not_null_or_nullable,
                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension3 {
                            dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                            dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                            dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                        },
                    }),
                    (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension4 { .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: NotNullOrNullable::NotNull,
                        postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.clone(),
                    }),
                }
            }
            generate_postgresql_json_type_record_handle_vec(PostgresqlJsonTypeRecordHandle {
                not_null_or_nullable: postgresql_json_type_record_element.not_null_or_nullable,
                postgresql_json_type_pattern: postgresql_json_type_record_element.postgresql_json_type_pattern,
            })
            .into_iter()
            .for_each(|postgresql_json_type_record_handle_element| {
                let postgresql_json_type_record = PostgresqlJsonTypeRecord {
                    postgresql_json_type: postgresql_json_type_record_element.postgresql_json_type.clone(),
                    not_null_or_nullable: postgresql_json_type_record_handle_element.not_null_or_nullable,
                    postgresql_json_type_pattern: postgresql_json_type_record_handle_element.postgresql_json_type_pattern,
                };
                if !acc.contains(&postgresql_json_type_record) {
                    acc.push(postgresql_json_type_record);
                }
            });
            acc
        })
    };
    // macros_helpers::write_string_into_file::write_string_into_file(
    //     "GeneratePostgresqlJsonTypesJsonVariants",
    //     &serde_json::to_string(&postgresql_json_type_record_vec).unwrap(),
    // );
    use rayon::iter::IntoParallelRefIterator;
    use rayon::iter::ParallelIterator;
    let (_fields_token_stream, postgresql_json_type_array) = postgresql_json_type_record_vec
        .into_iter()
        .enumerate()
        .collect::<std::vec::Vec<(std::primitive::usize, PostgresqlJsonTypeRecord)>>()
        .par_iter()
        // .into_iter() //just for console prints ordering
        .map(|(index, element)| {
            let postgresql_json_type = &element.postgresql_json_type;
            let not_null_or_nullable = &element.not_null_or_nullable;
            let postgresql_json_type_pattern = &element.postgresql_json_type_pattern;

            let rust_type_name = RustTypeName::from(postgresql_json_type);
            let postgresql_json_type_name = PostgresqlJsonTypeName::from(postgresql_json_type);

            let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();

            let value_snake_case = naming::ValueSnakeCase;
            let element_snake_case = naming::ElementSnakeCase;
            let as_upper_camel_case = naming::AsUpperCamelCase;
            let none_upper_camel_case = naming::NoneUpperCamelCase;
            let new_snake_case = naming::NewSnakeCase;
            let self_upper_camel_case = naming::SelfUpperCamelCase;
            let element_upper_camel_case = naming::ElementUpperCamelCase;
            let increment_snake_case = naming::IncrementSnakeCase;
            let query_snake_case = naming::QuerySnakeCase;
            let read_snake_case = naming::ReadSnakeCase;
            let error_snake_case = naming::ErrorSnakeCase;
            let create_vec_snake_case = naming::CreateVecSnakeCase;
            let option_update_snake_case = naming::OptionUpdateSnakeCase;
            let read_inner_vec_vec_snake_case = naming::ReadInnerVecVecSnakeCase;
            let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
            let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon;

            let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
            let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
            let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;

            let generate_ident_token_stream = |not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
                let vec_of_upper_camel_case = naming::VecOfUpperCamelCase;
                let array_of_upper_camel_case = naming::ArrayOfUpperCamelCase;
                let not_null_or_nullable_rust = not_null_or_nullable.rust();
                let (rust_part, postgresql_part) = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => (rust_type_name.to_string(), postgresql_json_type_name.to_string()),
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                        let d1 = dimension1_not_null_or_nullable;
                        let d1_rust = dimension1_not_null_or_nullable.rust();
                        (format!("{vec_of_upper_camel_case}{d1_rust}{rust_type_name}"), format!("{array_of_upper_camel_case}{d1}{postgresql_json_type_name}"))
                    }
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                        let d1 = dimension1_not_null_or_nullable;
                        let d1_rust = dimension1_not_null_or_nullable.rust();
                        let d2 = dimension2_not_null_or_nullable;
                        let d2_rust = dimension2_not_null_or_nullable.rust();
                        (format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{rust_type_name}"), format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{postgresql_json_type_name}"))
                    }
                    PostgresqlJsonTypePattern::ArrayDimension3 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                        dimension3_not_null_or_nullable,
                    } => {
                        let d1 = dimension1_not_null_or_nullable;
                        let d1_rust = dimension1_not_null_or_nullable.rust();
                        let d2 = dimension2_not_null_or_nullable;
                        let d2_rust = dimension2_not_null_or_nullable.rust();
                        let d3 = dimension3_not_null_or_nullable;
                        let d3_rust = dimension3_not_null_or_nullable.rust();
                        (
                            format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{rust_type_name}"),
                            format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{postgresql_json_type_name}"),
                        )
                    }
                    PostgresqlJsonTypePattern::ArrayDimension4 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                        dimension3_not_null_or_nullable,
                        dimension4_not_null_or_nullable,
                    } => {
                        let d1 = dimension1_not_null_or_nullable;
                        let d1_rust = dimension1_not_null_or_nullable.rust();
                        let d2 = dimension2_not_null_or_nullable;
                        let d2_rust = dimension2_not_null_or_nullable.rust();
                        let d3 = dimension3_not_null_or_nullable;
                        let d3_rust = dimension3_not_null_or_nullable.rust();
                        let d4 = dimension4_not_null_or_nullable;
                        let d4_rust = dimension4_not_null_or_nullable.rust();
                        (
                            format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{vec_of_upper_camel_case}{d4_rust}{rust_type_name}"),
                            format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{array_of_upper_camel_case}{d4}{postgresql_json_type_name}"),
                        )
                    }
                };
                format!("{not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{not_null_or_nullable}{postgresql_part}").parse::<proc_macro2::TokenStream>().unwrap()
            };
            let ident = &generate_ident_token_stream(not_null_or_nullable, postgresql_json_type_pattern);
            let ident_standart_not_null_upper_camel_case = &generate_ident_token_stream(&postgresql_crud_macros_common::NotNullOrNullable::NotNull, &PostgresqlJsonTypePattern::Standart);
            let ident_token_stream = quote::quote! {
                #[derive(Debug)]
                pub struct #ident;
            };
            let ident_standart_not_null_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident);
            let inner_type_standart_not_null_token_stream = match &postgresql_json_type {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => quote::quote! {std::primitive::i8},
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => quote::quote! {std::primitive::i16},
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => quote::quote! {std::primitive::i32},
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => quote::quote! {std::primitive::i64},
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => quote::quote! {std::primitive::u8},
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => quote::quote! {std::primitive::u16},
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => quote::quote! {std::primitive::u32},
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => quote::quote! {std::primitive::u64},
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => quote::quote! {std::primitive::f32},
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => quote::quote! {std::primitive::f64},
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => quote::quote! {std::primitive::bool},
                PostgresqlJsonType::StdStringStringAsJsonbString => quote::quote! {std::string::String},
                PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {uuid::Uuid},
            };
            let generate_current_ident_origin_non_wrapping = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(current_not_null_or_nullable, current_postgresql_json_type_pattern));
            let field_type_handle: &dyn quote::ToTokens = {
                let generate_current_ident_origin = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
                    let value = generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern);
                    match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&value),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&value),
                    }
                };
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => &inner_type_standart_not_null_token_stream,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                        let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &PostgresqlJsonTypePattern::Standart),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        };
                        generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    },
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => &{
                        let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable }),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        };
                        generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    },
                    PostgresqlJsonTypePattern::ArrayDimension3 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                        dimension3_not_null_or_nullable,
                    } => &{
                        let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                                dimension1_not_null_or_nullable,
                                &PostgresqlJsonTypePattern::ArrayDimension2 {
                                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                    dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                },
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        };
                        generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    },
                    PostgresqlJsonTypePattern::ArrayDimension4 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                        dimension3_not_null_or_nullable,
                        dimension4_not_null_or_nullable,
                    } => &{
                        let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                                dimension1_not_null_or_nullable,
                                &PostgresqlJsonTypePattern::ArrayDimension3 {
                                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                    dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                    dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                                },
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        };
                        generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    },
                }
            };
            let ident_origin_impl_new_value_type_token_stream = match &element.postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &inner_type_standart_not_null_token_stream,
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&inner_type_standart_not_null_token_stream),
                },
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote! {#inner_type_standart_not_null_token_stream});
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                },
                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => &{
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(quote::quote! {#inner_type_standart_not_null_token_stream});
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension2_type));
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                },
                PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                } => &{
                    let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(quote::quote! {#inner_type_standart_not_null_token_stream});
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension3_type));
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension2_type));
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                },
                PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                } => &{
                    let dimension4_type = dimension4_not_null_or_nullable.maybe_option_wrap(quote::quote! {#inner_type_standart_not_null_token_stream});
                    let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension4_type));
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension3_type));
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension2_type));
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                },
            };
            let ident_origin_token_stream = {
                let schema_name_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&ident_origin_upper_camel_case);
                let metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream = quote::quote! {
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
                let extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream = quote::quote! {schemars::Map::default()};
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
                enum SchemarsJsonSchema<'a> {
                    Derive,
                    Impl(SchemaObjectTokenStream<'a>),
                }
                let (instance_type_number_token_stream, instance_type_string_token_stream) = {
                    let generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type = |instance_type: &schemars::schema::InstanceType| {
                        let instance_type_token_stream: &dyn quote::ToTokens = match &instance_type {
                            schemars::schema::InstanceType::Null => &naming::NullUpperCamelCase,
                            schemars::schema::InstanceType::Boolean => &naming::BooleanUpperCamelCase,
                            schemars::schema::InstanceType::Object => &naming::ObjectUpperCamelCase,
                            schemars::schema::InstanceType::Array => &naming::ArrayUpperCamelCase,
                            schemars::schema::InstanceType::Number => &naming::NumberUpperCamelCase,
                            schemars::schema::InstanceType::String => &naming::StringUpperCamelCase,
                            schemars::schema::InstanceType::Integer => &naming::IntegerUpperCamelCase,
                        };
                        quote::quote! {Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::#instance_type_token_stream)))}
                    };
                    (
                        generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type(&schemars::schema::InstanceType::Number),
                        generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type(&schemars::schema::InstanceType::String),
                    )
                };
                let number_token_stream = quote::quote! {Some(Box::new(schemars::schema::NumberValidation {
                    multiple_of: None,
                    maximum: Some(#inner_type_standart_not_null_token_stream ::MAX as std::primitive::f64),
                    exclusive_maximum: None,
                    minimum: Some(#inner_type_standart_not_null_token_stream ::MIN as std::primitive::f64),
                    exclusive_minimum: None,
                }))};
                let string_token_stream = quote::quote! {Some(Box::new(schemars::schema::StringValidation {
                    max_length: Some(36),
                    min_length: Some(36),
                    pattern: None,
                }))};
                let schemars_json_schema = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) = (&not_null_or_nullable, &postgresql_json_type_pattern) {
                    let schema_object_token_stream_integer = SchemaObjectTokenStream {
                        metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream,
                        instance_type: &instance_type_number_token_stream,
                        format: &none_upper_camel_case,
                        enum_values: &none_upper_camel_case,
                        const_value: &none_upper_camel_case,
                        subschemas: &none_upper_camel_case,
                        number: &number_token_stream,
                        string: &none_upper_camel_case,
                        array: &none_upper_camel_case,
                        object: &none_upper_camel_case,
                        reference: &none_upper_camel_case,
                        extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream,
                    };
                    match &postgresql_json_type {
                        PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                        PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean | PostgresqlJsonType::StdStringStringAsJsonbString => SchemarsJsonSchema::Derive,
                        PostgresqlJsonType::UuidUuidAsJsonbString => SchemarsJsonSchema::Impl(SchemaObjectTokenStream {
                            metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream,
                            instance_type: &instance_type_string_token_stream,
                            format: &none_upper_camel_case,
                            enum_values: &none_upper_camel_case,
                            const_value: &none_upper_camel_case,
                            subschemas: &none_upper_camel_case,
                            number: &none_upper_camel_case,
                            string: &string_token_stream,
                            array: &none_upper_camel_case,
                            object: &none_upper_camel_case,
                            reference: &none_upper_camel_case,
                            extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream,
                        }),
                    }
                } else {
                    SchemarsJsonSchema::Derive
                };
                let (serde_serialize, serde_deserialize) = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) = (&not_null_or_nullable, &postgresql_json_type_pattern) {
                    let serde_serialize = match &postgresql_json_type {
                        PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                        | PostgresqlJsonType::StdStringStringAsJsonbString
                        | PostgresqlJsonType::UuidUuidAsJsonbString => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                    };
                    let serde_deserialize = match &postgresql_json_type {
                        PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                        | PostgresqlJsonType::StdStringStringAsJsonbString
                        | PostgresqlJsonType::UuidUuidAsJsonbString => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                    };
                    (serde_serialize, serde_deserialize)
                } else {
                    (postgresql_crud_macros_common::DeriveOrImpl::Derive, postgresql_crud_macros_common::DeriveOrImpl::Derive)
                };
                let ident_origin_token_stream = {
                    let maybe_derive_serde_serialize_token_stream = match &serde_serialize {
                        postgresql_crud_macros_common::DeriveOrImpl::Derive => quote::quote! {serde::Serialize,},
                        postgresql_crud_macros_common::DeriveOrImpl::Impl(_) => proc_macro2::TokenStream::new(),
                    };
                    let maybe_derive_serde_deserialize_token_stream = match &serde_deserialize {
                        postgresql_crud_macros_common::DeriveOrImpl::Derive => quote::quote! {serde::Deserialize,},
                        postgresql_crud_macros_common::DeriveOrImpl::Impl(_) => proc_macro2::TokenStream::new(),
                    };
                    let maybe_derive_schemars_json_schema_token_stream: &dyn quote::ToTokens = match &schemars_json_schema {
                        SchemarsJsonSchema::Derive => &quote::quote! {schemars::JsonSchema,},
                        SchemarsJsonSchema::Impl(_) => &proc_macro2_token_stream_new,
                    };
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            PartialOrd,
                            #maybe_derive_serde_serialize_token_stream
                            #maybe_derive_serde_deserialize_token_stream
                            utoipa::ToSchema,
                            #maybe_derive_schemars_json_schema_token_stream
                        )]
                        pub struct #ident_origin_upper_camel_case(#field_type_handle);
                    }
                };
                let impl_ident_origin_token_stream = {
                    let pub_fn_new_token_stream = {
                        let ident_origin_impl_new_self_content_token_stream = {
                            let generate_match_option_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                                quote::quote! {match value {
                                    Some(value) => Some(#type_token_stream::new(value)),
                                    None => None
                                }}
                            };
                            let generate_array_dimensions_initialization_token_stream = |type_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {value.into_iter().map(|element|#type_token_stream::new(element)).collect()},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&type_token_stream),
                            };
                            match &postgresql_json_type_pattern {
                                PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {value},
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                                },
                                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_array_dimensions_initialization_token_stream(&{
                                    let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&PostgresqlJsonTypePattern::Standart, dimension1_not_null_or_nullable),
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                                    };
                                    generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                }),
                                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => generate_array_dimensions_initialization_token_stream(&{
                                    let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable }, dimension1_not_null_or_nullable),
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                                    };
                                    generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                }),
                                PostgresqlJsonTypePattern::ArrayDimension3 {
                                    dimension1_not_null_or_nullable,
                                    dimension2_not_null_or_nullable,
                                    dimension3_not_null_or_nullable,
                                } => generate_array_dimensions_initialization_token_stream(&{
                                    let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                                            &PostgresqlJsonTypePattern::ArrayDimension2 {
                                                dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                                dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                            },
                                            dimension1_not_null_or_nullable,
                                        ),
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                                    };
                                    generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                }),
                                PostgresqlJsonTypePattern::ArrayDimension4 {
                                    dimension1_not_null_or_nullable,
                                    dimension2_not_null_or_nullable,
                                    dimension3_not_null_or_nullable,
                                    dimension4_not_null_or_nullable,
                                } => generate_array_dimensions_initialization_token_stream(&{
                                    let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                                            &PostgresqlJsonTypePattern::ArrayDimension3 {
                                                dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                                dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                                dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                                            },
                                            dimension1_not_null_or_nullable,
                                        ),
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                                    };
                                    generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                }),
                            }
                        };
                        quote::quote! {
                            pub fn new(#value_snake_case: #ident_origin_impl_new_value_type_token_stream) -> Self {
                                Self(#ident_origin_impl_new_self_content_token_stream)
                            }
                        }
                    };
                    let maybe_additional_uuid_standart_not_null_impl_token_stream = if let (PostgresqlJsonType::UuidUuidAsJsonbString, postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) = (&postgresql_json_type, &not_null_or_nullable, &postgresql_json_type_pattern) {
                        //todo need for uuid bind for json object logic//need for uuid bind for json object logic
                        let query_bind_as_postgresql_text_token_stream = quote::quote! {
                            pub fn query_bind_as_postgresql_text(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
                                sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                                std::string::String
                            > {
                                if let Err(error) = query.try_bind(self.0.to_string()) {
                                    return Err(error.to_string())
                                }
                                Ok(query)
                            }
                        };
                        let get_inner_token_stream = quote::quote! {
                            pub fn get_inner<'a>(&'a self) -> &'a #field_type_handle {
                                &self.0
                            }
                        };
                        quote::quote! {
                            #query_bind_as_postgresql_text_token_stream
                            #get_inner_token_stream
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    quote::quote! {
                        impl #ident_origin_upper_camel_case {
                            #pub_fn_new_token_stream
                            #maybe_additional_uuid_standart_not_null_impl_token_stream
                        }
                    }
                };
                let maybe_impl_schemars_json_schema_for_ident_origin_token_stream: &dyn quote::ToTokens = match &schemars_json_schema {
                    SchemarsJsonSchema::Derive => &proc_macro2_token_stream_new,
                    SchemarsJsonSchema::Impl(schema_object_token_stream) => &{
                        let schema_id_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("postgresql_crud::postgersql_json_type::{ident_origin_upper_camel_case}"));
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
                            impl schemars::JsonSchema for #ident_origin_upper_camel_case {
                                fn schema_name() -> std::string::String {
                                    #schema_name_format_handle_token_stream.to_owned()
                                }
                                fn schema_id() -> std::borrow::Cow<'static, str> {
                                    std::borrow::Cow::Borrowed(#schema_id_format_handle_token_stream)
                                }
                                fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
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
                    },
                };
                let maybe_impl_is_string_empty_for_ident_origin_token_stream = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) = (&not_null_or_nullable, &postgresql_json_type_pattern) {
                    match &postgresql_json_type {
                        PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => proc_macro2::TokenStream::new(),
                        PostgresqlJsonType::StdStringStringAsJsonbString => postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_token_stream(&ident_origin_upper_camel_case),
                        PostgresqlJsonType::UuidUuidAsJsonbString => postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_token_stream(&ident_origin_upper_camel_case),
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let maybe_impl_serde_serialize_for_ident_standart_not_null_origin_token_stream = match serde_serialize {
                    postgresql_crud_macros_common::DeriveOrImpl::Derive => proc_macro2::TokenStream::new(),
                    postgresql_crud_macros_common::DeriveOrImpl::Impl(value) => value,
                };
                let maybe_impl_serde_deserialize_for_ident_standart_not_null_origin_token_stream = match serde_deserialize {
                    postgresql_crud_macros_common::DeriveOrImpl::Derive => proc_macro2::TokenStream::new(),
                    postgresql_crud_macros_common::DeriveOrImpl::Impl(value) => value,
                };
                let impl_std_fmt_display_for_ident_origin_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {write!(formatter, "{self:?}")});
                let impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self:#?}")});
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_origin_upper_camel_case, &{
                    let content_token_stream = match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_type {
                                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                                | PostgresqlJsonType::StdStringStringAsJsonbString => quote::quote! {#core_default_default_default_token_stream},
                                PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {uuid::Uuid::new_v4()},
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {Some(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                        },
                        PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {vec![#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {Some(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                        },
                    };
                    quote::quote! {Self(#content_token_stream)}
                });
                let impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream = {
                    let sqlx_types_json_type_field_type_token_stream = postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&ident_origin_impl_new_value_type_token_stream);
                    quote::quote! {
                        impl sqlx::Type<sqlx::Postgres> for #ident_origin_upper_camel_case {
                            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                                <#sqlx_types_json_type_field_type_token_stream as sqlx::Type<sqlx::Postgres>>::type_info()
                            }
                            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
                                <#sqlx_types_json_type_field_type_token_stream as sqlx::Type<sqlx::Postgres>>::compatible(ty)
                            }
                        }
                    }
                };
                let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = {
                    quote::quote! {
                        impl sqlx::Encode<'_, sqlx::Postgres> for #ident_origin_upper_camel_case {
                            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
                            }
                        }
                    }
                };
                quote::quote! {
                    #ident_origin_token_stream
                    #impl_ident_origin_token_stream

                    #maybe_impl_schemars_json_schema_for_ident_origin_token_stream
                    #maybe_impl_is_string_empty_for_ident_origin_token_stream
                    #maybe_impl_serde_serialize_for_ident_standart_not_null_origin_token_stream
                    #maybe_impl_serde_deserialize_for_ident_standart_not_null_origin_token_stream
                    #impl_std_fmt_display_for_ident_origin_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream
                    #impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream
                }
            };
            let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
            let ident_select_token_stream = {
                let ident_select_token_stream = {
                    let content_token_stream = match ArrayDimension::try_from(postgresql_json_type_pattern) {
                        Ok(array_dimension) => {
                            let mut arguments_token_stream = vec![];
                            for element in 1..=array_dimension.to_usize() {
                                let dimension_number_pagination_token_stream = format!("dimension{element}_pagination").parse::<proc_macro2::TokenStream>().unwrap();
                                arguments_token_stream.push(quote::quote! {
                                    #dimension_number_pagination_token_stream: #import_path::PaginationStartsWithZero
                                });
                            }
                            quote::quote! {{
                                #(#arguments_token_stream),*
                            }}
                        }
                        Err(_) => quote::quote! {;},
                    };
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            serde::Serialize,
                            serde::Deserialize,
                            utoipa::ToSchema,
                            schemars::JsonSchema,
                        )]
                        pub struct #ident_select_upper_camel_case #content_token_stream
                    }
                };
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_select_upper_camel_case,
                    &match ArrayDimension::try_from(postgresql_json_type_pattern) {
                        Ok(array_dimension) => {
                            let mut arguments_token_stream = vec![];
                            for element in 1..=array_dimension.to_usize() {
                                let dimension_number_pagination_token_stream = format!("dimension{element}_pagination").parse::<proc_macro2::TokenStream>().unwrap();
                                arguments_token_stream.push(quote::quote! {
                                    #dimension_number_pagination_token_stream: #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                });
                            }
                            quote::quote! {Self {
                                #(#arguments_token_stream),*
                            }}
                        }
                        Err(_) => quote::quote! {Self{}},
                    },
                );
                quote::quote! {
                    #ident_select_token_stream
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream
                }
            };
            let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
            let ident_where_element_token_stream = match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                    let variants = {
                        #[derive(Debug, Clone)]
                        enum PostgresqlJsonTypeSpecific {
                            Number,
                            Bool,
                            String,
                        }
                        impl std::convert::From<&PostgresqlJsonType> for PostgresqlJsonTypeSpecific {
                            fn from(value: &PostgresqlJsonType) -> Self {
                                match value {
                                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                                    | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                                    | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                                    | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                                    | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                                    | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                                    | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                                    | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                                    | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                                    | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => Self::Number,
                                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::Bool,
                                    PostgresqlJsonType::StdStringStringAsJsonbString | PostgresqlJsonType::UuidUuidAsJsonbString => Self::String,
                                }
                            }
                        }
                        let postgresql_json_type_specific = PostgresqlJsonTypeSpecific::from(&element.postgresql_json_type);
                        let common_postgresql_json_type_filters = vec![postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal { ident: quote::quote! {#ident_origin_upper_camel_case} }];
                        let ident_origin_upper_camel_case_token_stream = quote::quote! {#ident_origin_upper_camel_case};
                        match &element.postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => {
                                let common_standart_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In { ident: ident_origin_upper_camel_case_token_stream.clone() });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut vec = common_standart_postgresql_json_type_filters.clone();
                                        vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan { ident: ident_origin_upper_camel_case_token_stream.clone() });
                                        vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between { ident: ident_origin_upper_camel_case_token_stream.clone() });
                                        vec
                                    }
                                    PostgresqlJsonTypeSpecific::Bool => common_standart_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut vec = common_standart_postgresql_json_type_filters.clone();
                                        vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::RegularExpression);
                                        vec
                                    }
                                }
                            }
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                                let array_dimension1_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension1_not_null_or_nullable, &PostgresqlJsonTypePattern::Standart));
                                    quote::quote! {#value}
                                };
                                let common_array_dimension1_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneOverlapsWithArray {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension1_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneGreaterThan {
                                            ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneBetween {
                                            ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementGreaterThan {
                                            ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan {
                                            ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters
                                    }
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension1_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension1_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsRegularExpression);
                                        filters
                                    }
                                }
                            }
                            PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                                let array_dimension1_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension1_not_null_or_nullable, &PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable }));
                                    quote::quote! {#value}
                                };
                                let array_dimension2_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension2_not_null_or_nullable, &PostgresqlJsonTypePattern::Standart));
                                    quote::quote! {#value}
                                };
                                let common_array_dimension2_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoOverlapsWithArray {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension2_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoGreaterThan {
                                            ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoBetween {
                                            ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElementGreaterThan {
                                            ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsGreaterThan {
                                            ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters
                                    }
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension2_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension2_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElementRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsRegularExpression);
                                        filters
                                    }
                                }
                            }
                            PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                            } => {
                                let array_dimension1_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        dimension1_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::ArrayDimension2 {
                                            dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                            dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                        },
                                    ));
                                    quote::quote! {#value}
                                };
                                let array_dimension2_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension2_not_null_or_nullable, &PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension3_not_null_or_nullable }));
                                    quote::quote! {#value}
                                };
                                let array_dimension3_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension3_not_null_or_nullable, &PostgresqlJsonTypePattern::Standart));
                                    quote::quote! {#value}
                                };
                                let common_array_dimension3_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeOverlapsWithArray {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension3_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeGreaterThan {
                                            ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeBetween {
                                            ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElementGreaterThan {
                                            ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsGreaterThan {
                                            ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters
                                    }
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension3_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension3_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElementRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsRegularExpression);
                                        filters
                                    }
                                }
                            }
                            PostgresqlJsonTypePattern::ArrayDimension4 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                                dimension4_not_null_or_nullable,
                            } => {
                                let array_dimension1_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        dimension1_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::ArrayDimension3 {
                                            dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                            dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                            dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                                        },
                                    ));
                                    quote::quote! {#value}
                                };
                                let array_dimension2_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        dimension2_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::ArrayDimension2 {
                                            dimension1_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                            dimension2_not_null_or_nullable: *dimension4_not_null_or_nullable,
                                        },
                                    ));
                                    quote::quote! {#value}
                                };
                                let array_dimension3_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension3_not_null_or_nullable, &PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension4_not_null_or_nullable }));
                                    quote::quote! {#value}
                                };
                                let array_dimension4_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension4_not_null_or_nullable, &PostgresqlJsonTypePattern::Standart));
                                    quote::quote! {#value}
                                };
                                let common_array_dimension4_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourEqual {
                                        ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray {
                                        ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourOverlapsWithArray {
                                        ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsEqual {
                                        ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourIn {
                                        ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone(),
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension4_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourGreaterThan {
                                            ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourBetween {
                                            ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementGreaterThan {
                                            ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsGreaterThan {
                                            ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters
                                    }
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension4_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension4_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsRegularExpression);
                                        filters
                                    }
                                }
                            }
                        }
                    };
                    postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                        &variants.iter().map(|element| element as &dyn postgresql_crud_macros_common::PostgresqlFilter).collect(),
                        &ident,
                        &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                        &postgresql_crud_macros_common::IsQueryBindMutable::False,
                    )
                }
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                    let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
                    let ident_as_token_stream = generate_ident_token_stream(&postgresql_crud_macros_common::NotNullOrNullable::NotNull, &element.postgresql_json_type_pattern);
                    quote::quote! {
                        pub type #ident_where_element_upper_camel_case = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter<
                            <#ident_as_token_stream as #import_path::PostgresqlJsonType>::WhereElement
                        >;
                    }
                }
            };
            // println!("{ident_where_element_token_stream}");
            //exists because need to implement .into_inner() for fields (only for read subtype)
            let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
            let ident_read_token_stream = {
                let ident_read_token_stream = quote::quote! {
                    #[derive(
                        Debug,
                        Clone,
                        PartialEq,
                        serde::Serialize,
                        serde::Deserialize,
                        utoipa::ToSchema,
                        schemars::JsonSchema,
                    )]
                    pub struct #ident_read_upper_camel_case(#ident_origin_upper_camel_case);
                };
                let impl_ident_read_token_stream = {
                    let pub_fn_new_token_stream = quote::quote! {
                        pub fn new(#value_snake_case: #ident_origin_impl_new_value_type_token_stream) -> Self {
                            Self(#ident_origin_upper_camel_case::new(#value_snake_case))
                        }
                    };
                    quote::quote! {
                        impl #ident_read_upper_camel_case {
                            #pub_fn_new_token_stream
                        }
                    }
                };
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream =
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_read_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
                quote::quote! {
                    #ident_read_token_stream
                    #impl_ident_read_token_stream
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream
                }
            };
            // println!("{ident_read_token_stream}");
            let ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident);
            let ident_read_only_ids_standart_not_null_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_read_only_ids_token_stream = {
                let content_token_stream = {
                    use postgresql_crud_macros_common::NotNullOrNullable;
                    let std_option_option_unit_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&quote::quote! {()});
                    let vec_token_stream = |ident_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&ident_token_stream);
                    let content_token_stream = if let PostgresqlJsonType::UuidUuidAsJsonbString = &element.postgresql_json_type {
                        match &element.postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => {
                                let token_stream1 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => inner_type_standart_not_null_token_stream.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&inner_type_standart_not_null_token_stream),
                                };
                                quote::quote! {#token_stream1}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                                let token_stream1 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => inner_type_standart_not_null_token_stream.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&inner_type_standart_not_null_token_stream),
                                });
                                let token_stream2 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                };
                                quote::quote! {#token_stream2}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                                let token_stream1 = vec_token_stream(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => inner_type_standart_not_null_token_stream.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&inner_type_standart_not_null_token_stream),
                                });
                                let token_stream2 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                });
                                let token_stream3 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream2.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream2),
                                };
                                quote::quote! {#token_stream3}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                            } => {
                                let token_stream1 = vec_token_stream(&match &dimension3_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => inner_type_standart_not_null_token_stream.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&inner_type_standart_not_null_token_stream),
                                });
                                let token_stream2 = vec_token_stream(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                });
                                let token_stream3 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream2.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream2),
                                });
                                let token_stream4 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream3.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream3),
                                };
                                quote::quote! {#token_stream4}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension4 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                                dimension4_not_null_or_nullable,
                            } => {
                                let token_stream1 = vec_token_stream(&match &dimension4_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => inner_type_standart_not_null_token_stream.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&inner_type_standart_not_null_token_stream),
                                });
                                let token_stream2 = vec_token_stream(&match &dimension3_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                });
                                let token_stream3 = vec_token_stream(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream2.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream2),
                                });
                                let token_stream4 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream3.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream3),
                                });
                                let token_stream5 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream4.clone(),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream4),
                                };
                                quote::quote! {#token_stream5}
                            }
                        }
                    } else {
                        std_option_option_unit_token_stream
                    };
                    quote::quote! {#import_path::Value<#content_token_stream>}
                };
                quote::quote! {
                    #[derive(
                        Debug,
                        Clone,
                        PartialEq,
                        serde::Serialize,
                        serde::Deserialize,
                    )]
                    pub struct #ident_read_only_ids_upper_camel_case(pub #content_token_stream);
                }
            };
            let ident_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident);
            let ident_read_inner_token_stream = quote::quote! {
                pub type #ident_read_inner_upper_camel_case = #ident_origin_impl_new_value_type_token_stream;
            };
            let postgresql_crud_macros_common_import_path_postgresql_crud_common = postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon;
            let impl_postgresql_json_type_for_ident_token_stream = {
                let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
                let generate_dimension_number_stringified = |dimensions_number: std::primitive::usize| format!("dimension{dimensions_number}");
                let generate_dimension_number_start_stringified = |dimensions_number: std::primitive::usize| format!("{}_start", generate_dimension_number_stringified(dimensions_number));
                let generate_dimension_number_end_stringified = |dimensions_number: std::primitive::usize| format!("{}_end", generate_dimension_number_stringified(dimensions_number));
                //todo maybe reuse it in a function(not in the proc macro)
                let select_only_created_or_updated_ids_query_part_token_stream = if let PostgresqlJsonTypePattern::Standart = &element.postgresql_json_type_pattern
                    && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &element.not_null_or_nullable
                    && let PostgresqlJsonType::UuidUuidAsJsonbString = &element.postgresql_json_type
                {
                    quote::quote! {
                        match #increment_snake_case.checked_add(1) {
                            Some(#value_snake_case) => {
                                *#increment_snake_case = #value_snake_case;
                                Ok(format!("'{field_ident}',jsonb_build_object('value',${increment}),"))
                            }
                            None => Err(#import_path::QueryPartErrorNamed::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                        }
                    }
                } else {
                    quote::quote! {Ok(format!("'{field_ident}',jsonb_build_object('value','null'::jsonb),"))}
                };
                let select_only_created_or_updated_ids_query_bind_token_stream = if let PostgresqlJsonTypePattern::Standart = &element.postgresql_json_type_pattern
                    && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &element.not_null_or_nullable
                    && let PostgresqlJsonType::UuidUuidAsJsonbString = &element.postgresql_json_type
                {
                    quote::quote! {
                        if let Err(#error_snake_case) = #query_snake_case.try_bind(#value_snake_case) {
                            return Err(#error_snake_case.to_string());
                        }
                        Ok(#query_snake_case)
                    }
                } else {
                    quote::quote! {Ok(#query_snake_case)}
                };
                postgresql_crud_macros_common::generate_impl_postgresql_json_type_token_stream(
                    &postgresql_crud_macros_common_import_path_postgresql_crud_common,
                    &ident,
                    &ident_origin_upper_camel_case,
                    &ident_origin_upper_camel_case,
                    &ident_origin_upper_camel_case,
                    &ident_select_upper_camel_case,
                    &match &element.postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::False,
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::False,
                        },
                        PostgresqlJsonTypePattern::ArrayDimension1 { .. } => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::True,
                        PostgresqlJsonTypePattern::ArrayDimension2 { .. } => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::True,
                        PostgresqlJsonTypePattern::ArrayDimension3 { .. } => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::True,
                        PostgresqlJsonTypePattern::ArrayDimension4 { .. } => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::True,
                    },
                    &postgresql_crud_macros_common::IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed::False,
                    &postgresql_crud_macros_common::IsSelectQueryPartIsPostgresqlTypeUsed::False,
                    &{
                        let format_handle = {
                            //last child dimension value does not matter - null or type - works both good
                            use postgresql_crud_macros_common::NotNullOrNullable;
                            let column_name_and_maybe_field_getter_field_ident = format!("{{{}}}->'{{field_ident}}'", naming::ColumnNameAndMaybeFieldGetterSnakeCase);
                            let format_handle = match ArrayDimension::try_from(postgresql_json_type_pattern) {
                                Ok(array_dimension) => {
                                    let generate_jsonb_agg = |jsonb_agg_content: &std::primitive::str, jsonb_array_elements_content: &std::primitive::str, ordinality_content: &std::primitive::str, dimensions_number: std::primitive::usize| {
                                        let dimension_number_start = generate_dimension_number_start_stringified(dimensions_number);
                                        let dimension_number_end = generate_dimension_number_end_stringified(dimensions_number);
                                        format!("select jsonb_agg(({jsonb_agg_content})) from jsonb_array_elements(({jsonb_array_elements_content})) with ordinality {ordinality_content} between {{{dimension_number_start}}} and {{{dimension_number_end}}}")
                                    };
                                    match ArrayDimensionSelectPattern::try_from(&array_dimension) {
                                        //Dimension1 does not fit into pattern. its only for 2+ dimensions
                                        Ok(array_dimension_select_pattern) => {
                                            let generate_d_number_elem = |content: std::primitive::usize| format!("d{content}_elem");
                                            let generate_d_number_ord = |content: std::primitive::usize| format!("d{content}_elem");
                                            let generate_dot_value = |content: &std::primitive::str| format!("{content}.value");
                                            let generate_as_value_where = |first_content: &std::primitive::str, second_content: &std::primitive::str| format!("as {first_content}(value, {second_content}) where {second_content}");
                                            let one = 1;
                                            generate_jsonb_agg(
                                                &{
                                                    let mut current_usize_value = array_dimension_select_pattern.to_usize();
                                                    array_dimension_select_pattern.select_array().into_iter().fold(generate_dot_value(&generate_d_number_elem(current_usize_value)), |mut acc, not_null_or_nullable| {
                                                        let current_usize_value_minus_one = current_usize_value - one;
                                                        let d_usize_minus_one_elem_value = generate_dot_value(&generate_d_number_elem(current_usize_value_minus_one));
                                                        let value = generate_jsonb_agg(&acc, &d_usize_minus_one_elem_value, &generate_as_value_where(&generate_d_number_elem(current_usize_value), &generate_d_number_ord(current_usize_value)), current_usize_value);
                                                        acc = match &not_null_or_nullable {
                                                            NotNullOrNullable::NotNull => value,
                                                            NotNullOrNullable::Nullable => format!("case when jsonb_typeof({d_usize_minus_one_elem_value})='array' then ({value}) else null end"),
                                                        };
                                                        current_usize_value = current_usize_value_minus_one;
                                                        acc
                                                    })
                                                },
                                                &column_name_and_maybe_field_getter_field_ident,
                                                &generate_as_value_where(&generate_d_number_elem(one), &generate_d_number_ord(one)),
                                                one,
                                            )
                                        }
                                        Err(_) => generate_jsonb_agg("value", &format!("select {column_name_and_maybe_field_getter_field_ident}"), "where ordinality", 1),
                                    }
                                }
                                Err(_) => column_name_and_maybe_field_getter_field_ident.clone(),
                            };
                            match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => format_handle,
                                NotNullOrNullable::Nullable => format!("case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident})='null' then 'null'::jsonb else ({format_handle}) end"),
                            }
                        };
                        let maybe_dimensions_start_end_initialization = {
                            let mut acc = vec![];
                            if let Ok(array_dimension) = ArrayDimension::try_from(postgresql_json_type_pattern) {
                                for element in 1..=array_dimension.to_usize() {
                                    let dimension_number_start_token_stream = generate_dimension_number_start_stringified(element).parse::<proc_macro2::TokenStream>().unwrap();
                                    let dimension_number_end_token_stream = generate_dimension_number_end_stringified(element).parse::<proc_macro2::TokenStream>().unwrap();
                                    let dimension_number_pagination_token_stream = format!("{}_pagination", generate_dimension_number_stringified(element)).parse::<proc_macro2::TokenStream>().unwrap();
                                    acc.push(quote::quote! {
                                        let #dimension_number_start_token_stream = #value_snake_case.#dimension_number_pagination_token_stream.start();
                                        let #dimension_number_end_token_stream = #value_snake_case.#dimension_number_pagination_token_stream.end();
                                    });
                                }
                            }
                            acc
                        };
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',({format_handle})))"));
                        quote::quote! {
                            #(#maybe_dimensions_start_end_initialization)*
                            format!(#format_handle_token_stream)
                        }
                    },
                    &ident_where_element_upper_camel_case,
                    &ident_read_upper_camel_case,
                    &ident_read_only_ids_upper_camel_case,
                    &if let PostgresqlJsonTypePattern::Standart = &element.postgresql_json_type_pattern
                        && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &element.not_null_or_nullable
                        && let PostgresqlJsonType::UuidUuidAsJsonbString = &element.postgresql_json_type
                    {
                        quote::quote! {format!("jsonb_build_object('value',{column_name_and_maybe_field_getter})")}
                    } else {
                        quote::quote! {"jsonb_build_object('value','null'::jsonb)".to_string()}
                    },
                    &ident_read_inner_upper_camel_case,
                    &{
                        let generate_match_element_zero_token_stream = |match_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {match #match_token_stream {
                                Some(#value_snake_case) => Some(#value_snake_case.0 #content_token_stream),
                                None => None
                            }}
                        };
                        let content_token_stream = {
                            let element_dot_zero_token_stream = quote::quote! {#element_snake_case.0};
                            let generate_into_iter_map_element_collect_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                                quote::quote! {.into_iter().map(|#element_snake_case|#content_token_stream).collect()}
                            };
                            let generate_into_iter_map_element_collect_not_null_or_nullable_token_stream = |not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable| {
                                let content_token_stream = match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => element_dot_zero_token_stream.clone(),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_element_zero_token_stream(&element_dot_zero_token_stream, &proc_macro2::TokenStream::new()),
                                };
                                generate_into_iter_map_element_collect_token_stream(&content_token_stream)
                            };
                            let generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream = |not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, content_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_into_iter_map_element_collect_token_stream(&quote::quote! {#element_dot_zero_token_stream #content_token_stream}),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let match_element_zero_token_stream = generate_match_element_zero_token_stream(&element_dot_zero_token_stream, &content_token_stream);
                                    quote::quote! {.into_iter().map(|#element_snake_case|#match_element_zero_token_stream).collect()}
                                }
                            };
                            match &element.postgresql_json_type_pattern {
                                PostgresqlJsonTypePattern::Standart => proc_macro2::TokenStream::new(),
                                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(dimension1_not_null_or_nullable),
                                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                                    let dimension2_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(dimension2_not_null_or_nullable);
                                    generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable_content_token_stream)
                                }
                                PostgresqlJsonTypePattern::ArrayDimension3 {
                                    dimension1_not_null_or_nullable,
                                    dimension2_not_null_or_nullable,
                                    dimension3_not_null_or_nullable,
                                } => {
                                    let dimension3_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(dimension3_not_null_or_nullable);
                                    let dimension2_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable_content_token_stream);
                                    generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable_content_token_stream)
                                }
                                PostgresqlJsonTypePattern::ArrayDimension4 {
                                    dimension1_not_null_or_nullable,
                                    dimension2_not_null_or_nullable,
                                    dimension3_not_null_or_nullable,
                                    dimension4_not_null_or_nullable,
                                } => {
                                    let dimension4_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(dimension4_not_null_or_nullable);
                                    let dimension3_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable_content_token_stream);
                                    let dimension2_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable_content_token_stream);
                                    generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable_content_token_stream)
                                }
                            }
                        };
                        let value_dot_zero_dot_zero_token_stream = quote::quote! {#value_snake_case.0.0};
                        match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#value_dot_zero_dot_zero_token_stream #content_token_stream},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_element_zero_token_stream(&value_dot_zero_dot_zero_token_stream, &content_token_stream),
                        }
                    },
                    &ident_origin_upper_camel_case,
                    &ident_origin_upper_camel_case,
                    &{
                        let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{jsonb_set_path}}}}}}',${{increment}})"));
                        quote::quote! {
                            match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    Ok(format!(#format_handle_token_stream))
                                }
                                None => Err(#import_path::QueryPartErrorNamed::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                            }
                        }
                    },
                    &postgresql_crud_macros_common::IsUpdateQueryPartSelfUpdateUsed::False,
                    &postgresql_crud_macros_common::IsUpdateQueryPartJsonbSetTargetUsed::False,
                    &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                    &quote::quote! {
                        if let Err(error) = query.try_bind(#value_snake_case) {
                            return Err(error.to_string());
                        }
                        Ok(query)
                    },
                    &select_only_created_or_updated_ids_query_part_token_stream,
                    &if let PostgresqlJsonTypePattern::Standart = &element.postgresql_json_type_pattern
                        && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &element.not_null_or_nullable
                        && let PostgresqlJsonType::UuidUuidAsJsonbString = &element.postgresql_json_type
                    {
                        postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::True
                    } else {
                        postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::False
                    },
                    &select_only_created_or_updated_ids_query_bind_token_stream,
                    &select_only_created_or_updated_ids_query_part_token_stream,
                    &if let PostgresqlJsonTypePattern::Standart = &element.postgresql_json_type_pattern
                        && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &element.not_null_or_nullable
                        && let PostgresqlJsonType::UuidUuidAsJsonbString = &element.postgresql_json_type
                    {
                        postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::True
                    } else {
                        postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::False
                    },
                    &select_only_created_or_updated_ids_query_bind_token_stream,
                )
            };
            let impl_postgresql_json_type_test_cases_for_ident_token_stream = {
                let generate_read_or_update_new_or_try_new_unwraped_for_test_token_stream = |read_or_update: &postgresql_crud_macros_common::ReadOrUpdate| {
                    let read_or_update_upper_camel_case = read_or_update.upper_camel_case();
                    quote::quote! {<#self_upper_camel_case::#element_upper_camel_case
                        as
                        #postgresql_crud_macros_common_import_path_postgresql_crud_common::#postgresql_json_type_upper_camel_case
                    >::#read_or_update_upper_camel_case::#new_snake_case(#value_snake_case)}
                };
                postgresql_crud_macros_common::generate_impl_postgresql_json_type_test_cases_for_ident_token_stream(
                    &quote::quote! {#[cfg(feature = "test-utils")]},
                    &postgresql_crud_macros_common_import_path_postgresql_crud_common,
                    &ident_origin_impl_new_value_type_token_stream,
                    &ident,
                    &{
                        use postgresql_crud_macros_common::NotNullOrNullable;
                        let generate_acc_content_token_stream = |not_null_or_nullable: &NotNullOrNullable, ident_token_stream: &dyn quote::ToTokens| {
                            let current_ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident_token_stream);
                            let (element_or_some_element_token_stream, maybe_push_none_token_stream) = match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => (quote::quote! {element}, proc_macro2::TokenStream::new()),
                                NotNullOrNullable::Nullable => (quote::quote! {Some(element)}, quote::quote! {acc.push(None);}),
                            };
                            quote::quote! {
                                let mut acc = vec![];
                                for element in <#ident_token_stream as #import_path::PostgresqlJsonTypeTestCases>::#read_inner_vec_vec_snake_case(&#current_ident_read_only_ids_upper_camel_case(read_only_ids.0.clone())) {
                                    acc.push(#element_or_some_element_token_stream);
                                }
                                #maybe_push_none_token_stream
                                vec![acc]
                            }
                        };
                        let content_token_stream = match &postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => {
                                    let content_token_stream = match &postgresql_json_type {
                                        PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => postgresql_crud_macros_common::generate_int_min_zero_max_test_vec_token_stream(&quote::quote! {std::primitive::i8}),
                                        PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => postgresql_crud_macros_common::std_primitive_i16_test_vec_token_stream(),
                                        PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => postgresql_crud_macros_common::std_primitive_i32_test_vec_token_stream(),
                                        PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => postgresql_crud_macros_common::std_primitive_i64_test_vec_token_stream(),
                                        PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => postgresql_crud_macros_common::generate_int_min_zero_max_test_vec_token_stream(&quote::quote! {std::primitive::u8}),
                                        PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => postgresql_crud_macros_common::generate_int_min_zero_max_test_vec_token_stream(&quote::quote! {std::primitive::u16}),
                                        PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => postgresql_crud_macros_common::generate_int_min_zero_max_test_vec_token_stream(&quote::quote! {std::primitive::u32}),
                                        PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => postgresql_crud_macros_common::generate_int_min_zero_max_test_vec_token_stream(&quote::quote! {std::primitive::u64}),
                                        PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => postgresql_crud_macros_common::std_primitive_f32_test_vec_token_stream(),
                                        PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => postgresql_crud_macros_common::std_primitive_f64_test_vec_token_stream(),
                                        PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => postgresql_crud_macros_common::std_primitive_bool_test_vec_token_stream(),
                                        PostgresqlJsonType::StdStringStringAsJsonbString => postgresql_crud_macros_common::std_string_string_test_vec_token_stream(),
                                        PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {vec![uuid::Uuid::new_v4()]},
                                    };
                                    quote::quote! {vec![#content_token_stream]}
                                }
                                NotNullOrNullable::Nullable => {
                                    quote::quote! {
                                        let mut acc = vec![];
                                        for element0 in <#ident_standart_not_null_upper_camel_case as #import_path::PostgresqlJsonTypeTestCases>::#read_inner_vec_vec_snake_case(&#ident_read_only_ids_standart_not_null_upper_camel_case(read_only_ids.0.clone())) {
                                            for element1 in element0 {
                                                acc.push(Some(element1));
                                            }
                                        }
                                        acc.push(None);
                                        vec![acc]
                                    }
                                }
                            },
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_acc_content_token_stream(not_null_or_nullable, &generate_ident_token_stream(dimension1_not_null_or_nullable, &PostgresqlJsonTypePattern::Standart)),
                            PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => generate_acc_content_token_stream(not_null_or_nullable, &generate_ident_token_stream(dimension1_not_null_or_nullable, &PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable })),
                            PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                            } => generate_acc_content_token_stream(
                                not_null_or_nullable,
                                &generate_ident_token_stream(
                                    dimension1_not_null_or_nullable,
                                    &PostgresqlJsonTypePattern::ArrayDimension2 {
                                        dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                        dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                    },
                                ),
                            ),
                            PostgresqlJsonTypePattern::ArrayDimension4 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                                dimension4_not_null_or_nullable,
                            } => generate_acc_content_token_stream(
                                not_null_or_nullable,
                                &generate_ident_token_stream(
                                    dimension1_not_null_or_nullable,
                                    &PostgresqlJsonTypePattern::ArrayDimension3 {
                                        dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                        dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                        dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                                    },
                                ),
                            ),
                        };
                        match &element.postgresql_json_type {
                            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => content_token_stream,
                            PostgresqlJsonType::StdStringStringAsJsonbString => content_token_stream,
                            PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {vec![]},
                        }
                    },
                    &generate_read_or_update_new_or_try_new_unwraped_for_test_token_stream(&postgresql_crud_macros_common::ReadOrUpdate::Read),
                    &generate_read_or_update_new_or_try_new_unwraped_for_test_token_stream(&postgresql_crud_macros_common::ReadOrUpdate::Update),
                    &if let PostgresqlJsonTypePattern::Standart = &element.postgresql_json_type_pattern
                        && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &element.not_null_or_nullable
                        && let PostgresqlJsonType::UuidUuidAsJsonbString = &element.postgresql_json_type
                    {
                        quote::quote! {Some(#import_path::Value { value: value.0.value })}
                    } else {
                        quote::quote! {
                            Some(#import_path::Value{
                                value: <#ident as #import_path::PostgresqlJsonType>::into_inner(
                                    <
                                        <#ident as #import_path::PostgresqlJsonType>::Read
                                        as
                                        #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                                    >::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                )
                            })
                        }
                    },
                    &{
                        use postgresql_crud_macros_common::NotNullOrNullable;
                        let none_token_stream = quote::quote! {None};
                        let content_token_stream = if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                            //todo remove useless .clone()
                            let generate_update_to_read_only_ids_token_stream = |ident_token_stream: &dyn quote::ToTokens, not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable| {
                                let content_token_stream: &dyn quote::ToTokens = match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &element_snake_case,
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &value_snake_case,
                                };
                                quote::quote! {
                                    <
                                        #ident_token_stream
                                        as
                                        #import_path::PostgresqlJsonTypeTestCases
                                    >::update_to_read_only_ids(&#content_token_stream).0.#value_snake_case
                                }
                            };
                            let generate_iter_or_match_token_stream = |not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, content_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                    value.0.clone().into_iter().map(|element|#content_token_stream).collect()
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                    match value.0.clone() {
                                        Some(value) => Some(#content_token_stream),
                                        None => None
                                    }
                                },
                            };
                            match &element.postgresql_json_type_pattern {
                                PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote::quote! {value.0},
                                    NotNullOrNullable::Nullable => generate_iter_or_match_token_stream(not_null_or_nullable, &generate_update_to_read_only_ids_token_stream(&generate_ident_token_stream(&postgresql_crud_macros_common::NotNullOrNullable::NotNull, &PostgresqlJsonTypePattern::Standart), not_null_or_nullable)),
                                },
                                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_iter_or_match_token_stream(
                                    not_null_or_nullable,
                                    &generate_update_to_read_only_ids_token_stream(
                                        &generate_ident_token_stream(
                                            &match &not_null_or_nullable {
                                                NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                                NotNullOrNullable::Nullable => NotNullOrNullable::NotNull,
                                            },
                                            &match &not_null_or_nullable {
                                                NotNullOrNullable::NotNull => PostgresqlJsonTypePattern::Standart,
                                                NotNullOrNullable::Nullable => PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable },
                                            },
                                        ),
                                        not_null_or_nullable,
                                    ),
                                ),
                                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => generate_iter_or_match_token_stream(
                                    not_null_or_nullable,
                                    &generate_update_to_read_only_ids_token_stream(
                                        &generate_ident_token_stream(
                                            &match &not_null_or_nullable {
                                                NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                                NotNullOrNullable::Nullable => NotNullOrNullable::NotNull,
                                            },
                                            &match &not_null_or_nullable {
                                                NotNullOrNullable::NotNull => PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable },
                                                NotNullOrNullable::Nullable => PostgresqlJsonTypePattern::ArrayDimension2 {
                                                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                                                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                                },
                                            },
                                        ),
                                        not_null_or_nullable,
                                    ),
                                ),
                                PostgresqlJsonTypePattern::ArrayDimension3 {
                                    dimension1_not_null_or_nullable,
                                    dimension2_not_null_or_nullable,
                                    dimension3_not_null_or_nullable,
                                } => generate_iter_or_match_token_stream(
                                    not_null_or_nullable,
                                    &generate_update_to_read_only_ids_token_stream(
                                        &generate_ident_token_stream(
                                            &match &not_null_or_nullable {
                                                NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                                NotNullOrNullable::Nullable => NotNullOrNullable::NotNull,
                                            },
                                            &match &not_null_or_nullable {
                                                NotNullOrNullable::NotNull => PostgresqlJsonTypePattern::ArrayDimension2 {
                                                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                                    dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                                },
                                                NotNullOrNullable::Nullable => PostgresqlJsonTypePattern::ArrayDimension3 {
                                                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                                                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                                    dimension3_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                                },
                                            },
                                        ),
                                        not_null_or_nullable,
                                    ),
                                ),
                                PostgresqlJsonTypePattern::ArrayDimension4 {
                                    dimension1_not_null_or_nullable,
                                    dimension2_not_null_or_nullable,
                                    dimension3_not_null_or_nullable,
                                    dimension4_not_null_or_nullable,
                                } => generate_iter_or_match_token_stream(
                                    not_null_or_nullable,
                                    &generate_update_to_read_only_ids_token_stream(
                                        &generate_ident_token_stream(
                                            &match &not_null_or_nullable {
                                                NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                                NotNullOrNullable::Nullable => NotNullOrNullable::NotNull,
                                            },
                                            &match &not_null_or_nullable {
                                                NotNullOrNullable::NotNull => PostgresqlJsonTypePattern::ArrayDimension3 {
                                                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                                    dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                                    dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                                                },
                                                NotNullOrNullable::Nullable => PostgresqlJsonTypePattern::ArrayDimension4 {
                                                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                                                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                                    dimension3_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                                    dimension4_not_null_or_nullable: *dimension4_not_null_or_nullable,
                                                },
                                            },
                                        ),
                                        not_null_or_nullable,
                                    ),
                                ),
                            }
                        } else {
                            none_token_stream
                        };
                        quote::quote! {#ident_read_only_ids_upper_camel_case(#import_path::Value{value: #content_token_stream})}
                    },
                    &if let PostgresqlJsonTypePattern::Standart = &element.postgresql_json_type_pattern
                        && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &element.not_null_or_nullable
                        && let PostgresqlJsonType::UuidUuidAsJsonbString = &element.postgresql_json_type
                    {
                        quote::quote! {Some(#import_path::Value { value: #ident_read_upper_camel_case::new(value.0.value)})}
                    } else {
                        quote::quote! {Some(#import_path::Value { value: #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream })}
                    },
                    &quote::quote!{match #option_update_snake_case {
                        Some(#value_snake_case) => #ident_read_upper_camel_case(#value_snake_case),
                        None => #read_snake_case
                    }},
                    &{
                        ////////////
                        use postgresql_crud_macros_common::NotNullOrNullable;
                        let generate_acc_content_token_stream = |not_null_or_nullable: &NotNullOrNullable, ident_token_stream: &dyn quote::ToTokens| {
                            let current_ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident_token_stream);
                            let (element_or_some_element_token_stream, maybe_push_none_token_stream) = match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => (quote::quote! {element}, proc_macro2::TokenStream::new()),
                                NotNullOrNullable::Nullable => (quote::quote! {Some(element)}, quote::quote! {acc.push(None);}),
                            };
                            quote::quote! {
                                let mut acc = vec![];
                                for element in <#ident_token_stream as #import_path::PostgresqlJsonTypeTestCases>::#read_inner_vec_vec_snake_case(&#current_ident_read_only_ids_upper_camel_case(read_only_ids.0.clone())) {
                                    acc.push(#element_or_some_element_token_stream);
                                }
                                #maybe_push_none_token_stream
                                vec![acc]
                            }
                        };
                        let content_token_stream = match &postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => {
                                    let content_token_stream = match &postgresql_json_type {
                                        PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => quote::quote!{std_primitive_i8_test_cases_vec},
                                        PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => quote::quote!{std_primitive_i16_test_cases_vec},
                                        PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => quote::quote!{std_primitive_i32_test_cases_vec},
                                        PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => quote::quote!{std_primitive_i64_test_cases_vec},
                                        PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => quote::quote!{std_primitive_u8_test_cases_vec},
                                        PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => quote::quote!{std_primitive_u16_test_cases_vec},
                                        PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => quote::quote!{std_primitive_u32_test_cases_vec},
                                        PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => quote::quote!{std_primitive_u64_test_cases_vec},
                                        PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => quote::quote!{std_primitive_f32_test_cases_vec},
                                        PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => quote::quote!{std_primitive_f64_test_cases_vec},
                                        PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => quote::quote!{std_primitive_bool_test_cases_vec},
                                        PostgresqlJsonType::StdStringStringAsJsonbString => quote::quote!{std_string_string_test_cases_vec},
                                        PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote!{uuid_uuid_test_cases_vec},
                                    };
                                    quote::quote!{
                                        postgresql_crud_common::#content_token_stream().into_iter().map(|element|
                                            <#ident as postgresql_crud_common::PostgresqlJsonType>::Create::new(element)
                                        ).collect()
                                    }
                                },
                                NotNullOrNullable::Nullable => {
                                    quote::quote! {
                                        {
                                            let mut acc = vec![];
                                            for element in <#ident_standart_not_null_upper_camel_case as #import_path::PostgresqlJsonTypeTestCases>::#create_vec_snake_case() {
                                                acc.push(<#ident as postgresql_crud_common::PostgresqlJsonType>::Create::new(Some(element.0)));
                                            }
                                            acc.push(<#ident as postgresql_crud_common::PostgresqlJsonType>::Create::new(None));
                                            acc
                                        }
                                    }
                                }
                            },
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                                // generate_acc_content_token_stream(not_null_or_nullable, &generate_ident_token_stream(dimension1_not_null_or_nullable, &PostgresqlJsonTypePattern::Standart))
                                quote::quote! {
                                    todo!()
                                }
                            },
                            PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                                // generate_acc_content_token_stream(not_null_or_nullable, &generate_ident_token_stream(dimension1_not_null_or_nullable, &PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable }))
                                quote::quote! {
                                    todo!()
                                }
                            },
                            PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                            } => {
                                // generate_acc_content_token_stream(
                                //     not_null_or_nullable,
                                //     &generate_ident_token_stream(
                                //         dimension1_not_null_or_nullable,
                                //         &PostgresqlJsonTypePattern::ArrayDimension2 {
                                //             dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                //             dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                //         },
                                //     )
                                // )
                                quote::quote! {
                                    todo!()
                                }
                            },
                            PostgresqlJsonTypePattern::ArrayDimension4 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                                dimension4_not_null_or_nullable,
                            } => {
                                // generate_acc_content_token_stream(
                                //     not_null_or_nullable,
                                //     &generate_ident_token_stream(
                                //         dimension1_not_null_or_nullable,
                                //         &PostgresqlJsonTypePattern::ArrayDimension3 {
                                //             dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                //             dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                //             dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                                //         },
                                //     )
                                // )
                                quote::quote! {
                                    todo!()
                                }
                            },
                        };
                        /////////////
                        match &element.postgresql_json_type {
                            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => content_token_stream,
                            PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => content_token_stream,
                            PostgresqlJsonType::StdStringStringAsJsonbString => content_token_stream,
                            PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {vec![]},
                        }
                    }
                )
            };
            let generated = quote::quote! {
                #ident_token_stream
                #ident_origin_token_stream
                #ident_select_token_stream
                #ident_where_element_token_stream
                #ident_read_token_stream
                #ident_read_only_ids_token_stream
                #ident_read_inner_token_stream
                #impl_postgresql_json_type_for_ident_token_stream
                #impl_postgresql_json_type_test_cases_for_ident_token_stream
            };
            (
                {
                    let field_ident = format!("field_{index}").parse::<proc_macro2::TokenStream>().unwrap();
                    quote::quote! {
                        pub #field_ident: #ident,
                    }
                    .to_string()
                },
                generated.to_string(),
            )
        })
        .collect::<(std::vec::Vec<String>, std::vec::Vec<String>)>();
    // let example_token_stream = {
    //     let fields_token_stream = fields_token_stream
    //         .into_iter()
    //         .map(|element| element.parse::<proc_macro2::TokenStream>().unwrap())
    //         .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    //     quote::quote! {
    //         pub struct GeneratePostgresqlJsonTypesExample {
    //             #(#fields_token_stream)*
    //         }
    //     }
    // };
    // if false {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "GeneratePostgresqlJsonTypesExample",
    //         &example_token_stream,
    //     );
    // }
    let generated = {
        let postgresql_json_type_array = postgresql_json_type_array.into_iter().map(|element| element.parse::<proc_macro2::TokenStream>().unwrap()).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            #(#postgresql_json_type_array)*
            // #example_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlJsonTypes",
    //     &generated
    // );
    generated.into()
}
