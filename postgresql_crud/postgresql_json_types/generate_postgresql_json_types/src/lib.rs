#[proc_macro]
pub fn generate_postgresql_json_types(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
    impl From<&PostgresqlJsonType> for RustTypeName {
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                naming::parameter::JsonbSelfUpperCamelCase::from_display(match &self {
                    Self::Number => &naming::NumberUpperCamelCase,
                    Self::Boolean => &naming::BooleanUpperCamelCase,
                    Self::String => &naming::StringUpperCamelCase,
                })
            )
        }
    }
    impl From<&PostgresqlJsonType> for PostgresqlJsonTypeName {
        fn from(value: &PostgresqlJsonType) -> Self {
            match &value {
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
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::Boolean,
                PostgresqlJsonType::StdStringStringAsJsonbString | PostgresqlJsonType::UuidUuidAsJsonbString => Self::String,
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
            self.to_string().parse::<proc_macro2::TokenStream>().expect("error eb6cafe0-ad0d-4108-8b0e-c062b155efbb").to_tokens(tokens);
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
    impl PostgresqlJsonTypePattern {
        const fn down_by_1(&self) -> Option<Self> {
            match &self {
                Self::Standart => None,
                Self::ArrayDimension1 { .. } => Some(Self::Standart),
                Self::ArrayDimension2 { dimension2_not_null_or_nullable, .. } => Some(Self::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable }),
                Self::ArrayDimension3 { dimension2_not_null_or_nullable, dimension3_not_null_or_nullable, .. } => Some(Self::ArrayDimension2 {
                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                }),
                Self::ArrayDimension4 {
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension3 {
                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                    dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                }),
            }
        }
        const fn down_by_2(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrayDimension1 { .. } => None,
                Self::ArrayDimension2 { .. } => Some(Self::Standart),
                Self::ArrayDimension3 { dimension3_not_null_or_nullable, .. } => Some(Self::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension3_not_null_or_nullable }),
                Self::ArrayDimension4 { dimension3_not_null_or_nullable, dimension4_not_null_or_nullable, .. } => Some(Self::ArrayDimension2 {
                    dimension1_not_null_or_nullable: *dimension3_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension4_not_null_or_nullable,
                }),
            }
        }
        const fn down_by_3(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrayDimension1 { .. } | Self::ArrayDimension2 { .. } => None,
                Self::ArrayDimension3 { .. } => Some(Self::Standart),
                Self::ArrayDimension4 { dimension4_not_null_or_nullable, .. } => Some(Self::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension4_not_null_or_nullable }),
            }
        }
        const fn down_by_4(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrayDimension1 { .. } | Self::ArrayDimension2 { .. } | Self::ArrayDimension3 { .. } => None,
                Self::ArrayDimension4 { .. } => Some(Self::Standart),
            }
        }
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
        const fn to_usize(&self) -> usize {
            match &self {
                Self::ArrayDimension1 { .. } => 1,
                Self::ArrayDimension2 { .. } => 2,
                Self::ArrayDimension3 { .. } => 3,
                Self::ArrayDimension4 { .. } => 4,
            }
        }
    }
    impl TryFrom<&PostgresqlJsonTypePattern> for ArrayDimension {
        type Error = ();
        fn try_from(value: &PostgresqlJsonTypePattern) -> Result<Self, Self::Error> {
            match &value {
                PostgresqlJsonTypePattern::Standart => Err(()),
                PostgresqlJsonTypePattern::ArrayDimension1 { .. } => Ok(Self::ArrayDimension1),
                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => Ok(Self::ArrayDimension2 { dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable }),
                PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable, .. } => Ok(Self::ArrayDimension3 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                }),
                PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    ..
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
        const fn to_usize(&self) -> usize {
            match &self {
                Self::ArrayDimension2 { .. } => 2,
                Self::ArrayDimension3 { .. } => 3,
                Self::ArrayDimension4 { .. } => 4,
            }
        }
        fn select_array(&self) -> Vec<&postgresql_crud_macros_common::NotNullOrNullable> {
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
    impl TryFrom<&ArrayDimension> for ArrayDimensionSelectPattern {
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
    #[derive(Debug, serde::Deserialize)]
    enum GeneratePostgresqlJsonTypesConfig {
        All,
        Concrete(Vec<PostgresqlJsonTypeRecord>),
    }
    use rayon::iter::IntoParallelRefIterator as _;
    use rayon::iter::ParallelIterator as _;
    panic_location::panic_location();
    let postgresql_json_type_record_vec = {
        let generate_postgresql_json_types_config = serde_json::from_str::<GeneratePostgresqlJsonTypesConfig>(&input_token_stream.to_string()).expect("failed to get Config for generate_postgresql_json_types");
        let postgresql_json_type_record_vec = match generate_postgresql_json_types_config {
            GeneratePostgresqlJsonTypesConfig::All => PostgresqlJsonType::into_array().into_iter().fold(vec![], |mut acc, postgresql_json_type| {
                for postgresql_json_type_pattern in PostgresqlJsonTypePattern::into_array() {
                    match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => {
                            for not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                acc.push(PostgresqlJsonTypeRecord {
                                    postgresql_json_type: postgresql_json_type.clone(),
                                    not_null_or_nullable,
                                    postgresql_json_type_pattern: postgresql_json_type_pattern.clone(),
                                });
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension1 { .. } => {
                            for not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                for dimension1_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                    acc.push(PostgresqlJsonTypeRecord {
                                        postgresql_json_type: postgresql_json_type.clone(),
                                        not_null_or_nullable,
                                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable },
                                    });
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension2 { .. } => {
                            for not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                for dimension1_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                    for dimension2_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                        acc.push(PostgresqlJsonTypeRecord {
                                            postgresql_json_type: postgresql_json_type.clone(),
                                            not_null_or_nullable,
                                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable },
                                        });
                                    }
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension3 { .. } => {
                            for not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                for dimension1_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                    for dimension2_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                        for dimension3_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                            acc.push(PostgresqlJsonTypeRecord {
                                                postgresql_json_type: postgresql_json_type.clone(),
                                                not_null_or_nullable,
                                                postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension3 {
                                                    dimension1_not_null_or_nullable,
                                                    dimension2_not_null_or_nullable,
                                                    dimension3_not_null_or_nullable,
                                                },
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension4 { .. } => {
                            for not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                for dimension1_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                    for dimension2_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                        for dimension3_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                            for dimension4_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
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
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                acc
            }),
            GeneratePostgresqlJsonTypesConfig::Concrete(value) => value,
        };
        {
            let mut seen = std::collections::HashSet::new();
            assert!(postgresql_json_type_record_vec.iter().all(|element| seen.insert(element)), "not unique postgersql type provided: {postgresql_json_type_record_vec:#?}");
        };
        postgresql_json_type_record_vec.into_iter().fold(vec![], |mut acc, postgresql_json_type_record_element| {
            use postgresql_crud_macros_common::NotNullOrNullable;
            #[derive(Clone)]
            struct PostgresqlJsonTypeRecordHandle {
                not_null_or_nullable: NotNullOrNullable,
                postgresql_json_type_pattern: PostgresqlJsonTypePattern,
            }
            fn generate_postgresql_json_type_record_handle_vec(postgresql_json_type_record_handle: PostgresqlJsonTypeRecordHandle) -> Vec<PostgresqlJsonTypeRecordHandle> {
                let generate_vec = |current_postgresql_json_type_record_handle: PostgresqlJsonTypeRecordHandle| generate_postgresql_json_type_record_handle_vec(current_postgresql_json_type_record_handle).into_iter().chain(std::iter::once(postgresql_json_type_record_handle.clone())).collect();
                match (&postgresql_json_type_record_handle.not_null_or_nullable, &postgresql_json_type_record_handle.postgresql_json_type_pattern) {
                    (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) => vec![postgresql_json_type_record_handle],
                    (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::Standart) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: NotNullOrNullable::NotNull,
                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::Standart,
                    }),
                    (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: *dimension1_not_null_or_nullable,
                        postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.down_by_1().expect("error 0e970a4f-eeec-421e-aa30-f90fc536d388"),
                    }),
                    (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: *dimension1_not_null_or_nullable,
                        postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.down_by_1().expect("error 85f8ed83-aa1f-4f52-8f8f-80aeb86f8083"),
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
                    (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                        not_null_or_nullable: NotNullOrNullable::NotNull,
                        postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.clone(),
                    }),
                }
            }
            for postgresql_json_type_record_handle_element in generate_postgresql_json_type_record_handle_vec(PostgresqlJsonTypeRecordHandle {
                not_null_or_nullable: postgresql_json_type_record_element.not_null_or_nullable,
                postgresql_json_type_pattern: postgresql_json_type_record_element.postgresql_json_type_pattern,
            }) {
                let postgresql_json_type_record = PostgresqlJsonTypeRecord {
                    postgresql_json_type: postgresql_json_type_record_element.postgresql_json_type.clone(),
                    not_null_or_nullable: postgresql_json_type_record_handle_element.not_null_or_nullable,
                    postgresql_json_type_pattern: postgresql_json_type_record_handle_element.postgresql_json_type_pattern,
                };
                if !acc.contains(&postgresql_json_type_record) {
                    acc.push(postgresql_json_type_record);
                }
            }
            acc
        })
    };
    // macros_helpers::write_string_into_file::write_string_into_file(
    //     "GeneratePostgresqlJsonTypesJsonVariants",
    //     &serde_json::to_string(&postgresql_json_type_record_vec).expect("error 5480b910-d98e-414a-a273-1a9bf44cc515"),
    // );
    let (_fields_token_stream, postgresql_json_type_array) = postgresql_json_type_record_vec
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, PostgresqlJsonTypeRecord)>>()
        .par_iter()
        // .into_iter() //just for console prints ordering
        .map(|(index, element)| {
            enum IsStandartNotNull {
                True,
                False,
            }
            enum IsStandartNotNullUuid {
                True,
                False,
            }
            struct SchemaObjectTokenStream<'lifetime> {
                metadata: &'lifetime dyn quote::ToTokens,
                instance_type: &'lifetime dyn quote::ToTokens,
                format: &'lifetime dyn quote::ToTokens,
                enum_values: &'lifetime dyn quote::ToTokens,
                const_value: &'lifetime dyn quote::ToTokens,
                subschemas: &'lifetime dyn quote::ToTokens,
                number: &'lifetime dyn quote::ToTokens,
                string: &'lifetime dyn quote::ToTokens,
                array: &'lifetime dyn quote::ToTokens,
                object: &'lifetime dyn quote::ToTokens,
                reference: &'lifetime dyn quote::ToTokens,
                extensions: &'lifetime dyn quote::ToTokens,
            }
            enum SchemarsJsonSchema<'schema_object_token_stream_tifetime> {
                Derive,
                Impl(SchemaObjectTokenStream<'schema_object_token_stream_tifetime>),
            }
            let postgresql_json_type = &element.postgresql_json_type;
            let not_null_or_nullable = &element.not_null_or_nullable;
            let postgresql_json_type_pattern = &element.postgresql_json_type_pattern;
            let rust_type_name = RustTypeName::from(postgresql_json_type);
            let postgresql_json_type_name = PostgresqlJsonTypeName::from(postgresql_json_type);
            let is_standart_not_null = if let (PostgresqlJsonTypePattern::Standart, postgresql_crud_macros_common::NotNullOrNullable::NotNull) = (&postgresql_json_type_pattern, &not_null_or_nullable) {
                IsStandartNotNull::True
            } else {
                IsStandartNotNull::False
            };
            let is_standart_not_null_uuid = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart, PostgresqlJsonType::UuidUuidAsJsonbString) = (&not_null_or_nullable, &postgresql_json_type_pattern, &postgresql_json_type) {
                IsStandartNotNullUuid::True
            } else {
                IsStandartNotNullUuid::False
            };
            let value_snake_case = naming::ValueSnakeCase;
            let element_snake_case = naming::ElementSnakeCase;
            let as_upper_camel_case = naming::AsUpperCamelCase;
            let none_upper_camel_case = naming::NoneUpperCamelCase;
            let new_snake_case = naming::NewSnakeCase;
            let self_upper_camel_case = naming::SelfUpperCamelCase;
            let increment_snake_case = naming::IncrementSnakeCase;
            let query_snake_case = naming::QuerySnakeCase;
            let read_snake_case = naming::ReadSnakeCase;
            let error_snake_case = naming::ErrorSnakeCase;
            let option_vec_create_snake_case = naming::OptionVecCreateSnakeCase;
            let option_update_snake_case = naming::OptionUpdateSnakeCase;
            let read_only_ids_to_two_dimensional_vec_read_inner_snake_case = naming::ReadOnlyIdsToTwoDimensionalVecReadInnerSnakeCase;
            let acc_snake_case = naming::AccSnakeCase;
            let create_snake_case = naming::CreateSnakeCase;
            let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
            let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
            let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon;
            let create_for_query_upper_camel_case = naming::CreateForQueryUpperCamelCase;
            let update_for_query_upper_camel_case = naming::UpdateForQueryUpperCamelCase;
            let update_upper_camel_case = naming::UpdateUpperCamelCase;
            let self_snake_case = naming::SelfSnakeCase;
            let equal_upper_camel_case = naming::EqualUpperCamelCase;
            let read_inner_upper_camel_case = naming::ReadInnerUpperCamelCase;
            let read_only_ids_merged_with_create_into_read_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoReadSnakeCase;
            let read_only_ids_merged_with_create_into_where_equal_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoWhereEqualSnakeCase;
            let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSnakeCase;

            let std_primitive_i8_token_stream = token_patterns::StdPrimitiveI8;
            let std_primitive_i16_token_stream = token_patterns::StdPrimitiveI16;
            let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
            let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
            let std_primitive_u8_token_stream = token_patterns::StdPrimitiveU8;
            let std_primitive_u16_token_stream = token_patterns::StdPrimitiveU16;
            let std_primitive_u32_token_stream = token_patterns::StdPrimitiveU32;
            let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
            let std_primitive_f32_token_stream = token_patterns::StdPrimitiveF32;
            let std_primitive_f64_token_stream = token_patterns::StdPrimitiveF64;
            let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
            let std_string_string_token_stream = token_patterns::StdStringString;
            let uuid_uuid_token_stream = token_patterns::UuidUuid;
            let serde_serialize_token_stream = token_patterns::SerdeSerialize;
            let serde_deserialize_token_stream = token_patterns::SerdeDeserialize;
            let utoipa_to_schema_token_stream = token_patterns::UtoipaToSchema;
            let schemars_json_schema_token_stream = token_patterns::SchemarsJsonSchema;

            let none_token_stream = quote::quote! {None};

            let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
            let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
            let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
            let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSizeCall;

            let generate_import_path_value_initialization_token_stream = |content_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_value_initialization_token_stream(&import_path, &content_token_stream);
            let generate_ident_token_stream = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
                let vec_of_upper_camel_case = naming::VecOfUpperCamelCase;
                let array_of_upper_camel_case = naming::ArrayOfUpperCamelCase;
                let not_null_or_nullable_rust = current_not_null_or_nullable.rust();
                let (rust_part, postgresql_part) = match &current_postgresql_json_type_pattern {
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
                format!("{not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{current_not_null_or_nullable}{postgresql_part}").parse::<proc_macro2::TokenStream>().expect("error 998d1471-be98-4669-8bd3-ca6c4a1a5853")
            };
            let ident = &generate_ident_token_stream(not_null_or_nullable, postgresql_json_type_pattern);
            let ident_standart_not_null_upper_camel_case = &generate_ident_token_stream(&postgresql_crud_macros_common::NotNullOrNullable::NotNull, &PostgresqlJsonTypePattern::Standart);
            let ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident);
            let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
            let ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident);
            let ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident);
            let ident_not_null_token_stream = generate_ident_token_stream(&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern);
            let ident_token_stream = {
                let ident_token_stream = quote::quote! {
                    #[derive(Debug)]
                    pub struct #ident;
                };
                quote::quote! {
                    #ident_token_stream
                }
            };
            let ident_standart_not_null_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident);
            let ident_read_inner_standart_not_null_alias_token_stream = {
                let content_token_stream: &dyn quote::ToTokens = match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => &std_primitive_i8_token_stream,
                    PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => &std_primitive_i16_token_stream,
                    PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => &std_primitive_i32_token_stream,
                    PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => &std_primitive_i64_token_stream,
                    PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => &std_primitive_u8_token_stream,
                    PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => &std_primitive_u16_token_stream,
                    PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => &std_primitive_u32_token_stream,
                    PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => &std_primitive_u64_token_stream,
                    PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => &std_primitive_f32_token_stream,
                    PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => &std_primitive_f64_token_stream,
                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => &std_primitive_bool_token_stream,
                    PostgresqlJsonType::StdStringStringAsJsonbString => &std_string_string_token_stream,
                    PostgresqlJsonType::UuidUuidAsJsonbString => &uuid_uuid_token_stream,
                };
                quote::quote! {#content_token_stream}
            };
            let generate_current_ident_origin_non_wrapping = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(current_not_null_or_nullable, current_postgresql_json_type_pattern));
            let generate_into_inner_content_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                let generate_match_element_zero_token_stream = |match_token_stream: &dyn quote::ToTokens, current_content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {match #match_token_stream {
                        Some(#value_snake_case) => Some(#value_snake_case.0 #current_content_token_stream),
                        None => None
                    }}
                };
                let element_dot_zero_token_stream = quote::quote! {#element_snake_case.0};
                let generate_into_iter_map_element_collect_token_stream = |current_content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {.into_iter().map(|#element_snake_case|#current_content_token_stream).collect()}
                };
                let generate_into_iter_map_element_collect_not_null_or_nullable_token_stream = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable| {
                    generate_into_iter_map_element_collect_token_stream(&match &current_not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => element_dot_zero_token_stream.clone(),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_element_zero_token_stream(&element_dot_zero_token_stream, &proc_macro2::TokenStream::new()),
                    })
                };
                let generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream = |
                    current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable,
                    current_content_token_stream: &dyn quote::ToTokens
                | {
                    match &current_not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_into_iter_map_element_collect_token_stream(&quote::quote! {#element_dot_zero_token_stream #current_content_token_stream}),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let match_element_zero_token_stream = generate_match_element_zero_token_stream(&element_dot_zero_token_stream, &current_content_token_stream);
                            quote::quote! {.into_iter().map(|#element_snake_case|#match_element_zero_token_stream).collect()}
                        }
                    }
                };
                let into_inner_content_token_stream = match &postgresql_json_type_pattern {
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
                };
                match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#content_token_stream #into_inner_content_token_stream},
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_element_zero_token_stream(&content_token_stream, &into_inner_content_token_stream),
                }
            };
            let ident_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident);
            let generate_pub_fn_new_value_ident_read_inner_content_token_stream = |content_token_stream: &dyn quote::ToTokens| macros_helpers::generate_pub_new_token_stream(&quote::quote! {#value_snake_case: #ident_read_inner_upper_camel_case}, &content_token_stream);
            let ident_create_for_query_upper_camel_case = naming::parameter::SelfCreateForQueryUpperCamelCase::from_tokens(&ident);
            let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
            let ident_update_for_query_upper_camel_case = naming::parameter::SelfUpdateForQueryUpperCamelCase::from_tokens(&ident);
            let ident_origin_token_stream = {
                let schema_name_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&ident_origin_upper_camel_case);
                //todo json schema logic
                let metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream = quote::quote! {
                    Some(Box::new(schemars::schema::Metadata {
                        id: None,
                        title: Some(#schema_name_format_handle_token_stream.to_owned()),
                        description: None,
                        default: None,
                        deprecated: false,
                        read_only: false,
                        write_only: false,
                        examples: Vec::default(),
                    }))
                };
                let extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream = quote::quote! {schemars::Map::default()};
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
                let number_validation_token_stream = quote::quote! {Some(Box::new(schemars::schema::NumberValidation {
                    multiple_of: None,
                    maximum: Some(#ident_read_inner_standart_not_null_alias_token_stream ::MAX as #std_primitive_f64_token_stream),
                    exclusive_maximum: None,
                    minimum: Some(#ident_read_inner_standart_not_null_alias_token_stream ::MIN as #std_primitive_f64_token_stream),
                    exclusive_minimum: None,
                }))};
                let string_validation_token_stream = quote::quote! {Some(Box::new(schemars::schema::StringValidation {
                    max_length: Some(36),
                    min_length: Some(36),
                    pattern: None,
                }))};
                let schemars_json_schema = if let IsStandartNotNull::True = &is_standart_not_null {
                    let schema_object_token_stream_integer = SchemaObjectTokenStream {
                        metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream,
                        instance_type: &instance_type_number_token_stream,
                        format: &none_upper_camel_case,
                        enum_values: &none_upper_camel_case,
                        const_value: &none_upper_camel_case,
                        subschemas: &none_upper_camel_case,
                        number: &number_validation_token_stream,
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
                            string: &string_validation_token_stream,
                            array: &none_upper_camel_case,
                            object: &none_upper_camel_case,
                            reference: &none_upper_camel_case,
                            extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream,
                        }),
                    }
                } else {
                    SchemarsJsonSchema::Derive
                };
                let (serde_serialize, serde_deserialize) = if let IsStandartNotNull::True = &is_standart_not_null {
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
                        postgresql_crud_macros_common::DeriveOrImpl::Derive => quote::quote! {#serde_serialize_token_stream,},
                        postgresql_crud_macros_common::DeriveOrImpl::Impl(_) => proc_macro2::TokenStream::new(),
                    };
                    let maybe_derive_serde_deserialize_token_stream = match &serde_deserialize {
                        postgresql_crud_macros_common::DeriveOrImpl::Derive => quote::quote! {serde::Deserialize,},
                        postgresql_crud_macros_common::DeriveOrImpl::Impl(_) => proc_macro2::TokenStream::new(),
                    };
                    let maybe_derive_schemars_json_schema_token_stream: &dyn quote::ToTokens = match &schemars_json_schema {
                        SchemarsJsonSchema::Derive => &quote::quote! {#schemars_json_schema_token_stream,},
                        SchemarsJsonSchema::Impl(_) => &proc_macro2::TokenStream::new(),
                    };
                    let content_token_stream: &dyn quote::ToTokens = {
                        let generate_current_ident_origin = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
                            let value = generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern);
                            match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&value),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&value),
                            }
                        };
                        match &postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => &ident_read_inner_standart_not_null_alias_token_stream,
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                            },
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                                let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error e994797d-7334-4e30-b180-af24c16b68b1")),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                };
                                generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                            },
                            PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => &{
                                let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 76eb44e3-3099-4c9a-a935-da3e6f6e4210")),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                };
                                generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                            },
                            PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => &{
                                let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 1b996c86-0b08-476a-b963-373dd6838496")),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                };
                                generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                            },
                            PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => &{
                                let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error d24b7481-27d9-40c0-8476-344a16d08f27")),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                };
                                generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                            },
                        }
                    };
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            PartialOrd,
                            #maybe_derive_serde_serialize_token_stream
                            #maybe_derive_serde_deserialize_token_stream
                            #utoipa_to_schema_token_stream,
                            #maybe_derive_schemars_json_schema_token_stream
                        )]
                        struct #ident_origin_upper_camel_case(#content_token_stream);
                    }
                };
                let ident_origin_impl_new_self_content_token_stream = {
                    let generate_match_option_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {match #value_snake_case {
                            Some(#value_snake_case) => Some(#type_token_stream::#new_snake_case(#value_snake_case)),
                            None => None
                        }}
                    };
                    let generate_array_dimensions_initialization_token_stream = |type_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#value_snake_case.into_iter().map(|#element_snake_case|#type_token_stream::#new_snake_case(#element_snake_case)).collect()},
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&type_token_stream),
                    };
                    match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {value},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                        },
                        PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_array_dimensions_initialization_token_stream(&{
                            let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("error 1160d3df-2e2b-4a33-a297-6b48546b9ca8"), dimension1_not_null_or_nullable),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                            };
                            generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                        }),
                        PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_array_dimensions_initialization_token_stream(&{
                            let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("error 8ab62f4e-611b-4228-a295-3e731e934b9c"), dimension1_not_null_or_nullable),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                            };
                            generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                        }),
                        PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_array_dimensions_initialization_token_stream(&{
                            let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("error ed64919d-4679-45da-9d14-22ddee84716b"), dimension1_not_null_or_nullable),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                            };
                            generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                        }),
                        PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_array_dimensions_initialization_token_stream(&{
                            let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("error 25646d29-5a30-49fb-b91a-7b49ed8c5b0a"), dimension1_not_null_or_nullable),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                            };
                            generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                        }),
                    }
                };
                let impl_ident_origin_token_stream = {
                    let pub_fn_new_token_stream = generate_pub_fn_new_value_ident_read_inner_content_token_stream(&quote::quote! {Self(#ident_origin_impl_new_self_content_token_stream)});
                    quote::quote! {
                        impl #ident_origin_upper_camel_case {
                            #pub_fn_new_token_stream
                        }
                    }
                };
                let impl_std_convert_from_ident_create_for_ident_origin_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(&ident_create_upper_camel_case, &ident_origin_upper_camel_case, &quote::quote! {#value_snake_case.0});
                let impl_std_convert_into_ident_read_inner_for_ident_origin_token_stream = {
                    let content_token_stream = generate_into_inner_content_token_stream(&quote::quote! {self.0});
                    quote::quote! {
                        impl Into<#ident_read_inner_upper_camel_case> for #ident_origin_upper_camel_case {
                            fn into(self) -> #ident_read_inner_upper_camel_case {
                                #content_token_stream
                            }
                        }
                    }
                };
                let impl_std_convert_from_ident_update_for_ident_origin_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(&ident_update_upper_camel_case, &ident_origin_upper_camel_case, &quote::quote! {#value_snake_case.0});
                let maybe_impl_schemars_json_schema_for_ident_origin_token_stream: &dyn quote::ToTokens = match &schemars_json_schema {
                    SchemarsJsonSchema::Derive => &proc_macro2::TokenStream::new(),
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
                        //todo maybe reuse
                        quote::quote! {
                            impl #schemars_json_schema_token_stream for #ident_origin_upper_camel_case {
                                fn schema_name() -> String {
                                    #schema_name_format_handle_token_stream.to_owned()
                                }
                                fn schema_id() -> std::borrow::Cow<'static, str> {
                                    std::borrow::Cow::Borrowed(#schema_id_format_handle_token_stream)
                                }
                                fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
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
                    },
                };
                let maybe_impl_is_string_empty_for_ident_origin_token_stream = if let IsStandartNotNull::True = &is_standart_not_null {
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
                        PostgresqlJsonType::StdStringStringAsJsonbString | PostgresqlJsonType::UuidUuidAsJsonbString => postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_token_stream(&ident_origin_upper_camel_case),
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
                let impl_std_fmt_display_for_ident_origin_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {write!(f, "{self:?}")});
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
                let impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_origin_upper_camel_case, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&ident_read_inner_upper_camel_case));
                let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_origin_upper_camel_case, &quote::quote! {sqlx::types::Json(&#self_snake_case.0)});
                quote::quote! {
                    #ident_origin_token_stream
                    #impl_ident_origin_token_stream
                    #impl_std_convert_from_ident_create_for_ident_origin_token_stream
                    #impl_std_convert_into_ident_read_inner_for_ident_origin_token_stream
                    #impl_std_convert_from_ident_update_for_ident_origin_token_stream

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
            let pub_fn_new_value_ident_read_inner_self_ident_origin_new_value_token_stream = generate_pub_fn_new_value_ident_read_inner_content_token_stream(&quote::quote! {Self(#ident_origin_upper_camel_case::new(#value_snake_case))});
            let ident_table_type_declaration_token_stream = {
                let ident_table_type_declaration_token_stream = {
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            PartialOrd,//maybe add it to the trait?
                            #serde_serialize_token_stream,
                            #serde_deserialize_token_stream,
                            #utoipa_to_schema_token_stream,
                            #schemars_json_schema_token_stream
                        )]
                        pub struct #ident_table_type_declaration_upper_camel_case(#ident_origin_upper_camel_case);
                    }
                };
                let impl_ident_table_type_declaration_token_stream = {
                    quote::quote! {
                        impl #ident_table_type_declaration_upper_camel_case {
                            #pub_fn_new_value_ident_read_inner_self_ident_origin_new_value_token_stream
                        }
                    }
                };
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_token_stream =
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_table_type_declaration_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
                //todo maybe add to trait?
                let impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_table_type_declaration_upper_camel_case, &quote::quote! {&#self_snake_case.0});
                //todo maybe add to trait?
                let impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_table_type_declaration_upper_camel_case, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&ident_read_inner_upper_camel_case));
                quote::quote! {
                    #ident_table_type_declaration_token_stream
                    #impl_ident_table_type_declaration_token_stream
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_token_stream
                    #impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_token_stream
                }
            };
            let ident_create_token_stream = {
                let ident_create_token_stream = {
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            #serde_serialize_token_stream,
                            #serde_deserialize_token_stream,
                            #utoipa_to_schema_token_stream,
                            #schemars_json_schema_token_stream
                        )]
                        pub struct #ident_create_upper_camel_case(#ident_origin_upper_camel_case);
                    }
                };
                let impl_ident_create_token_stream = {
                    quote::quote! {
                        impl #ident_create_upper_camel_case {
                            #pub_fn_new_value_ident_read_inner_self_ident_origin_new_value_token_stream
                        }
                    }
                };
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream =
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_create_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
                quote::quote! {
                    #ident_create_token_stream
                    #impl_ident_create_token_stream
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream
                }
            };
            let ident_create_for_query_token_stream = {
                let ident_create_for_query_token_stream = {
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            #serde_serialize_token_stream
                        )]
                        pub struct #ident_create_for_query_upper_camel_case(#ident_origin_upper_camel_case);
                    }
                };
                let impl_ident_create_for_query_token_stream = {
                    quote::quote! {
                        impl #ident_create_for_query_upper_camel_case {
                            #pub_fn_new_value_ident_read_inner_self_ident_origin_new_value_token_stream
                        }
                    }
                };
                let impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_create_for_query_upper_camel_case, &quote::quote! {sqlx::types::Json(&#self_snake_case.0)});
                let impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_create_for_query_upper_camel_case, &ident_origin_upper_camel_case);
                let impl_std_convert_from_ident_create_for_ident_create_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(&ident_create_upper_camel_case, &ident_create_for_query_upper_camel_case, &quote::quote! {Self(#value_snake_case.0)});
                let maybe_impl_std_convert_from_ident_update_for_ident_create_for_query_token_stream = if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                    macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(&ident_update_upper_camel_case, &ident_create_for_query_upper_camel_case, &quote::quote! {Self(#value_snake_case.0)})
                } else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_create_for_query_token_stream
                    #impl_ident_create_for_query_token_stream
                    #impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_token_stream
                    #impl_std_convert_from_ident_create_for_ident_create_for_query_token_stream
                    #maybe_impl_std_convert_from_ident_update_for_ident_create_for_query_token_stream
                }
            };
            let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
            let ident_select_token_stream = {
                let ident_select_token_stream = {
                    let content_token_stream = ArrayDimension::try_from(postgresql_json_type_pattern).map_or_else(
                        |()| quote::quote! {;},
                        |array_dimension| {
                            let mut arguments_token_stream = vec![];
                            for current_element in 1..=array_dimension.to_usize() {
                                let dimension_number_pagination_token_stream = format!("dimension{current_element}_pagination").parse::<proc_macro2::TokenStream>().expect("error 2ad1faf7-57a8-4cfb-8b71-0082b06436ea");
                                arguments_token_stream.push(quote::quote! {
                                    #dimension_number_pagination_token_stream: #import_path::PaginationStartsWithZero
                                });
                            }
                            quote::quote! {{#(#arguments_token_stream),*}}
                        }
                    );
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            #serde_serialize_token_stream,
                            #serde_deserialize_token_stream,
                            #utoipa_to_schema_token_stream,
                            #schemars_json_schema_token_stream,
                        )]
                        pub struct #ident_select_upper_camel_case #content_token_stream
                    }
                };
                let generate_default_some_one_content_token_stream = |default_some_one_or_default_some_one_with_max_page_size: &postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize| {
                    let content_token_stream = ArrayDimension::try_from(postgresql_json_type_pattern).map_or_else(
                        |()| proc_macro2::TokenStream::new(),
                        |array_dimension| {
                            let content_token_stream: &dyn quote::ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                                postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream,
                            };
                            let mut arguments_token_stream = vec![];
                            for current_element in 1..=array_dimension.to_usize() {
                                let dimension_number_pagination_token_stream = format!("dimension{current_element}_pagination").parse::<proc_macro2::TokenStream>().expect("error 26ca29fb-fd98-466a-a380-974a6c5d4166");
                                arguments_token_stream.push(quote::quote! {
                                    #dimension_number_pagination_token_stream: #content_token_stream
                                });
                            }
                            quote::quote! {#(#arguments_token_stream),*}
                        }
                    );
                    quote::quote! {Self{#content_token_stream}}
                };
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream =
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_upper_camel_case, &generate_default_some_one_content_token_stream(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne));
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_postgresql_json_type_ident_select_token_stream =
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(&ident_select_upper_camel_case, &generate_default_some_one_content_token_stream(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize));
                quote::quote! {
                    #ident_select_token_stream
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_postgresql_json_type_ident_select_token_stream
                }
            };
            let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
            let ident_where_token_stream = match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_postgresql_type_where_token_stream(
                    &{
                        #[derive(Debug, Clone)]
                        enum PostgresqlJsonTypeSpecific {
                            Number,
                            Bool,
                            String,
                        }
                        impl From<&PostgresqlJsonType> for PostgresqlJsonTypeSpecific {
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
                        let postgresql_json_type_specific = PostgresqlJsonTypeSpecific::from(postgresql_json_type);
                        let common_postgresql_json_type_filters = vec![postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal { ident: quote::quote! {#ident_table_type_declaration_upper_camel_case} }];
                        let ident_table_type_declaration_upper_camel_case_token_stream = quote::quote! {#ident_table_type_declaration_upper_camel_case};
                        match &postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => {
                                let common_standart_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters;
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In {
                                        ident: ident_table_type_declaration_upper_camel_case_token_stream.clone(),
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut vec = common_standart_postgresql_json_type_filters;
                                        vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan {
                                            ident: ident_table_type_declaration_upper_camel_case_token_stream.clone(),
                                        });
                                        vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between { ident: ident_table_type_declaration_upper_camel_case_token_stream });
                                        vec
                                    }
                                    PostgresqlJsonTypeSpecific::Bool => common_standart_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut vec = common_standart_postgresql_json_type_filters;
                                        vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::RegularExpression);
                                        vec
                                    }
                                }
                            }
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                                let array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case = {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 21eaebaf-cd7a-4625-9232-0e23788a5432")));
                                    quote::quote! {#value}
                                };
                                let common_array_dimension1_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters;
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneOverlapsWithArray {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension1_postgresql_json_type_filters;
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneGreaterThan {
                                            ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneBetween {
                                            ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementGreaterThan {
                                            ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan {
                                            ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case,
                                        });
                                        filters
                                    }
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension1_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension1_postgresql_json_type_filters;
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsRegularExpression);
                                        filters
                                    }
                                }
                            }
                            PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                                let array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case = {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 0c4491c4-8364-4c27-9478-227aefadb086")));
                                    quote::quote! {#value}
                                };
                                let array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case = {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension2_not_null_or_nullable, &postgresql_json_type_pattern.down_by_2().expect("error 2d4ee5d4-490e-4503-91a7-ed29f73e6219")));
                                    quote::quote! {#value}
                                };
                                let common_array_dimension2_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters;
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoOverlapsWithArray {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case,
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension2_postgresql_json_type_filters;
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoGreaterThan {
                                            ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoBetween {
                                            ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElementGreaterThan {
                                            ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsGreaterThan {
                                            ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case,
                                        });
                                        filters
                                    }
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension2_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension2_postgresql_json_type_filters;
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
                                let array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case = {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 3450bef4-e5f3-4bcd-b2de-4e4c67143336")));
                                    quote::quote! {#value}
                                };
                                let array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case = {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension2_not_null_or_nullable, &postgresql_json_type_pattern.down_by_2().expect("error 3c0d10f4-6d7d-45d0-b929-5e307c7d79b1")));
                                    quote::quote! {#value}
                                };
                                let array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case = {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension3_not_null_or_nullable, &postgresql_json_type_pattern.down_by_3().expect("error 9aaf9e82-0a92-4848-bfd4-de49013972a5")));
                                    quote::quote! {#value}
                                };
                                let common_array_dimension3_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters;
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeOverlapsWithArray {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case,
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case,
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension3_postgresql_json_type_filters;
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeGreaterThan {
                                            ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeBetween {
                                            ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElementGreaterThan {
                                            ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsGreaterThan {
                                            ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case,
                                        });
                                        filters
                                    }
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension3_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension3_postgresql_json_type_filters;
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
                                let array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case = {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 550d313b-e925-496d-8a57-87931c573155")));
                                    quote::quote! {#value}
                                };
                                let array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case = {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension2_not_null_or_nullable, &postgresql_json_type_pattern.down_by_2().expect("error 7bda1424-64c0-402e-9bf8-44d5fb3b9903")));
                                    quote::quote! {#value}
                                };
                                let array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case = {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension3_not_null_or_nullable, &postgresql_json_type_pattern.down_by_3().expect("error b43aa5bd-9bba-4f3e-b93b-a41f108262ff")));
                                    quote::quote! {#value}
                                };
                                let array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case = {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension4_not_null_or_nullable, &postgresql_json_type_pattern.down_by_4().expect("error a246885a-ca72-4e37-a396-b7220e237c7e")));
                                    quote::quote! {#value}
                                };
                                let common_array_dimension4_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters;
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourEqual {
                                        ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthGreaterThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray {
                                        ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourOverlapsWithArray {
                                        ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsEqual {
                                        ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case,
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case,
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case,
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourIn {
                                        ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension4_postgresql_json_type_filters;
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourGreaterThan {
                                            ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourBetween {
                                            ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementGreaterThan {
                                            ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                        });
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsGreaterThan {
                                            ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case,
                                        });
                                        filters
                                    }
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension4_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension4_postgresql_json_type_filters;
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsRegularExpression);
                                        filters
                                    }
                                }
                            }
                        }
                    }
                    .iter()
                    .map(|current_element| {
                        let current_element_handle: &dyn postgresql_crud_macros_common::PostgresqlFilter = current_element;
                        current_element_handle
                    })
                    .collect(),
                    &ident,
                    &postgresql_crud_macros_common::ShouldDeriveUtoipaToSchema::True,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                    &postgresql_crud_macros_common::IsQueryBindMutable::False,
                ),
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                    pub type #ident_where_upper_camel_case = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter<
                        <#ident_not_null_token_stream as #import_path::PostgresqlJsonType>::Where
                    >;
                }
            };
            //exists because need to implement .into_inner() for fields (only for read subtype)
            let ident_read_token_stream = {
                //todo maybe add some derive\impl to trait
                let ident_read_token_stream = quote::quote! {
                    #[derive(
                        Debug,
                        Clone,
                        PartialEq,
                        PartialOrd,
                        #serde_serialize_token_stream,
                        #serde_deserialize_token_stream,
                        #utoipa_to_schema_token_stream,
                        #schemars_json_schema_token_stream,
                    )]
                    pub struct #ident_read_upper_camel_case(#ident_origin_upper_camel_case);
                };
                let impl_ident_read_token_stream = {
                    quote::quote! {
                        impl #ident_read_upper_camel_case {
                            #pub_fn_new_value_ident_read_inner_self_ident_origin_new_value_token_stream
                        }
                    }
                };
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream =
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_read_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
                let impl_sqlx_encode_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_read_upper_camel_case, &quote::quote! {&#self_snake_case.0});
                let impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_read_upper_camel_case, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&ident_read_inner_upper_camel_case));
                quote::quote! {
                    #ident_read_token_stream
                    #impl_ident_read_token_stream
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream
                    #impl_sqlx_encode_sqlx_postgres_for_ident_read_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream
                }
            };
            let ident_read_only_ids_standart_not_null_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_read_only_ids_token_stream = {
                let content_token_stream = {
                    use postgresql_crud_macros_common::NotNullOrNullable;
                    let std_option_option_unit_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&quote::quote! {()});
                    let vec_token_stream = |value: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&value);
                    let content_token_stream = if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                        match &postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => {
                                let token_stream1 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                                };
                                quote::quote! {#token_stream1}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                                let token_stream1 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                                });
                                let token_stream2 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                };
                                quote::quote! {#token_stream2}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                                let token_stream1 = vec_token_stream(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                                });
                                let token_stream2 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                });
                                let token_stream3 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream2,
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
                                    NotNullOrNullable::NotNull => quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                                });
                                let token_stream2 = vec_token_stream(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                });
                                let token_stream3 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream2,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream2),
                                });
                                let token_stream4 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream3,
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
                                    NotNullOrNullable::NotNull => quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                                });
                                let token_stream2 = vec_token_stream(&match &dimension3_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                });
                                let token_stream3 = vec_token_stream(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream2,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream2),
                                });
                                let token_stream4 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream3,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream3),
                                });
                                let token_stream5 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream4,
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
                        #serde_serialize_token_stream,
                        #serde_deserialize_token_stream,
                    )]
                    pub struct #ident_read_only_ids_upper_camel_case(pub #content_token_stream);
                }
            };
            let ident_read_inner_token_stream = {
                let type_token_stream = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => &ident_read_inner_standart_not_null_alias_token_stream,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                        let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream});
                        not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                    },
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => &{
                        let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream});
                        let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension2_type));
                        not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                    },
                    PostgresqlJsonTypePattern::ArrayDimension3 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                        dimension3_not_null_or_nullable,
                    } => &{
                        let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream});
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
                        let dimension4_type = dimension4_not_null_or_nullable.maybe_option_wrap(quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream});
                        let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension4_type));
                        let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension3_type));
                        let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension2_type));
                        not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                    },
                };
                quote::quote! {
                    pub type #ident_read_inner_upper_camel_case = #type_token_stream;
                }
            };
            let ident_update_token_stream = {
                let ident_update_token_stream = {
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            #serde_serialize_token_stream,
                            #serde_deserialize_token_stream,
                            #utoipa_to_schema_token_stream,
                            #schemars_json_schema_token_stream
                        )]
                        pub struct #ident_update_upper_camel_case(#ident_origin_upper_camel_case);
                    }
                };
                let impl_ident_update_token_stream = {
                    quote::quote! {
                        impl #ident_update_upper_camel_case {
                            #pub_fn_new_value_ident_read_inner_self_ident_origin_new_value_token_stream
                        }
                    }
                };
                let impl_error_occurence_lib_to_std_string_string_for_ident_update_token_stream = if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                    macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_update_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self:?}")})
                } else {
                    proc_macro2::TokenStream::new()
                };
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream =
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_update_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
                quote::quote! {
                    #ident_update_token_stream
                    #impl_ident_update_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_update_token_stream
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream
                }
            };
            let ident_update_for_query_token_stream = {
                let ident_update_for_query_token_stream = {
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            #serde_serialize_token_stream,
                        )]
                        pub struct #ident_update_for_query_upper_camel_case(#ident_origin_upper_camel_case);
                    }
                };
                let impl_ident_update_for_query_token_stream = {
                    quote::quote! {
                        impl #ident_update_for_query_upper_camel_case {
                            #pub_fn_new_value_ident_read_inner_self_ident_origin_new_value_token_stream
                        }
                    }
                };
                let impl_std_convert_from_ident_update_for_ident_update_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(&ident_update_upper_camel_case, &ident_update_for_query_upper_camel_case, &quote::quote! {Self(#value_snake_case.0)});
                //its only for primitive json types
                let impl_sqlx_encode_sqlx_postgres_for_ident_update_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_update_for_query_upper_camel_case, &quote::quote! {sqlx::types::Json(&#self_snake_case.0)});
                let impl_sqlx_type_sqlx_postgres_for_ident_update_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_update_for_query_upper_camel_case, &ident_origin_upper_camel_case);
                quote::quote! {
                    #ident_update_for_query_token_stream
                    #impl_ident_update_for_query_token_stream
                    #impl_std_convert_from_ident_update_for_ident_update_for_query_token_stream
                    #impl_sqlx_encode_sqlx_postgres_for_ident_update_for_query_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_update_for_query_token_stream
                }
            };
            let postgresql_crud_macros_common_import_path_postgresql_crud_common = postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon;
            let impl_postgresql_json_type_for_ident_token_stream = {
                let generate_dimension_number_stringified = |dimensions_number: usize| format!("dimension{dimensions_number}");
                let generate_dimension_number_start_stringified = |dimensions_number: usize| format!("{}_start", generate_dimension_number_stringified(dimensions_number));
                let generate_dimension_number_end_stringified = |dimensions_number: usize| format!("{}_end", generate_dimension_number_stringified(dimensions_number));
                let select_only_created_or_updated_ids_query_part_token_stream = if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                    quote::quote! {
                        match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                            Ok(#value_snake_case) => Ok(format!("'{field_ident}',jsonb_build_object('value',${value}),")),
                            Err(#error_snake_case) => Err(#error_snake_case),
                        }
                    }
                } else {
                    quote::quote! {Ok(field_ident_jsonb_build_object_value(&field_ident))}
                };
                let select_only_created_or_updated_ids_query_bind_token_stream = if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
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
                    &ident_table_type_declaration_upper_camel_case,
                    &ident_create_upper_camel_case,
                    &ident_create_for_query_upper_camel_case,
                    &ident_select_upper_camel_case,
                    &match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::False,
                        PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::True,
                    },
                    &postgresql_crud_macros_common::IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed::False,
                    &postgresql_crud_macros_common::IsSelectQueryPartIsPostgresqlTypeUsed::False,
                    &{
                        let format_handle = {
                            //last child dimension value does not matter - null or type - works both good
                            use postgresql_crud_macros_common::NotNullOrNullable;
                            let column_name_and_maybe_field_getter_field_ident = format!("{{{}}}->'{{field_ident}}'", naming::ColumnNameAndMaybeFieldGetterSnakeCase);
                            let format_handle = ArrayDimension::try_from(postgresql_json_type_pattern).map_or_else(
                                |()| column_name_and_maybe_field_getter_field_ident.clone(),
                                |array_dimension| {
                                    let generate_jsonb_agg = |jsonb_agg_content: &str, jsonb_array_elements_content: &str, ordinality_content: &str, dimensions_number: usize| {
                                        let dimension_number_start = generate_dimension_number_start_stringified(dimensions_number);
                                        let dimension_number_end = generate_dimension_number_end_stringified(dimensions_number);
                                        format!(
                                            "select jsonb_agg(({jsonb_agg_content})) from jsonb_array_elements(({jsonb_array_elements_content})) with ordinality {ordinality_content} between {{{dimension_number_start}}} and {{{dimension_number_end}}}"
                                        )
                                    };
                                    ArrayDimensionSelectPattern::try_from(&array_dimension).map_or_else(
                                        |()| generate_jsonb_agg(
                                            "value",
                                            &format!("select {column_name_and_maybe_field_getter_field_ident}"),
                                            "where ordinality",
                                            1,
                                        ),
                                        |array_dimension_select_pattern| {
                                            // Dimension1 does not fit into pattern. its only for 2+ dimensions
                                            let generate_d_number_elem = |content: usize| format!("d{content}_elem");
                                            let generate_d_number_ord = |content: usize| format!("d{content}_elem");
                                            let generate_dot_value = |content: &str| format!("{content}.value");
                                            let generate_as_value_where =
                                                |first_content: &str, second_content: &str| format!("as {first_content}(value, {second_content}) where {second_content}");
                                            let one = 1;
                                            generate_jsonb_agg(
                                                &{
                                                    let mut current_usize_value = array_dimension_select_pattern.to_usize();
                                                    array_dimension_select_pattern
                                                        .select_array()
                                                        .into_iter()
                                                        .fold(generate_dot_value(&generate_d_number_elem(current_usize_value)), |mut acc, current_not_null_or_nullable| {
                                                            let current_usize_value_minus_one = current_usize_value.checked_sub(one).expect("error a35e873e-a2a1-4a25-8de1-c35dbb0b65af");
                                                            let d_usize_minus_one_elem_value = generate_dot_value(&generate_d_number_elem(current_usize_value_minus_one));
                                                            let value = generate_jsonb_agg(
                                                                &acc,
                                                                &d_usize_minus_one_elem_value,
                                                                &generate_as_value_where(&generate_d_number_elem(current_usize_value), &generate_d_number_ord(current_usize_value)),
                                                                current_usize_value,
                                                            );
                                                            acc = match &current_not_null_or_nullable {
                                                                NotNullOrNullable::NotNull => value,
                                                                NotNullOrNullable::Nullable => {
                                                                    format!("case when jsonb_typeof({d_usize_minus_one_elem_value})='array' then ({value}) else null end")
                                                                }
                                                            };
                                                            current_usize_value = current_usize_value_minus_one;
                                                            acc
                                                        })
                                                },
                                                &column_name_and_maybe_field_getter_field_ident,
                                                &generate_as_value_where(&generate_d_number_elem(one), &generate_d_number_ord(one)),
                                                one,
                                            )
                                        },
                                    )
                                },
                            );
                            match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => format_handle,
                                NotNullOrNullable::Nullable => format!("case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident})='null' then 'null'::jsonb else ({format_handle}) end"),
                            }
                        };
                        let maybe_dimensions_start_end_initialization = {
                            let mut acc = vec![];
                            if let Ok(array_dimension) = ArrayDimension::try_from(postgresql_json_type_pattern) {
                                for current_element in 1..=array_dimension.to_usize() {
                                    let dimension_number_start_token_stream = generate_dimension_number_start_stringified(current_element).parse::<proc_macro2::TokenStream>().expect("error 77ec13b9-710a-460f-9239-ac9c3680244d");
                                    let dimension_number_end_token_stream = generate_dimension_number_end_stringified(current_element).parse::<proc_macro2::TokenStream>().expect("error 24acbb5e-c0fe-4257-b299-8746887ce198");
                                    let dimension_number_pagination_token_stream = format!("{}_pagination", generate_dimension_number_stringified(current_element)).parse::<proc_macro2::TokenStream>().expect("error 745c99b3-4e24-46c2-a671-9c7b4dce76f4");
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
                            Ok(format!(#format_handle_token_stream))
                        }
                    },
                    &ident_where_upper_camel_case,
                    &ident_read_upper_camel_case,
                    &ident_read_only_ids_upper_camel_case,
                    &{
                        let content_token_stream = if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                            quote::quote! {format!("jsonb_build_object('value',{column_name_and_maybe_field_getter})")}
                        } else {
                            quote::quote! {"jsonb_build_object('value','null'::jsonb)".to_string()}
                        };
                        quote::quote! {Ok(#content_token_stream)}
                    },
                    &ident_read_inner_upper_camel_case,
                    &generate_into_inner_content_token_stream(&quote::quote! {#value_snake_case.0.0}),
                    &ident_update_upper_camel_case,
                    &ident_update_for_query_upper_camel_case,
                    &{
                        let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{jsonb_set_path}}}}}}',${{increment}})"));
                        quote::quote! {
                            match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                                Ok(#value_snake_case) => Ok(format!(#format_handle_token_stream)),
                                Err(#error_snake_case) => Err(#error_snake_case),
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
                    &if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                        postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::True
                    } else {
                        postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::False
                    },
                    &select_only_created_or_updated_ids_query_bind_token_stream,
                    &select_only_created_or_updated_ids_query_part_token_stream,
                    &if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                        postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::True
                    } else {
                        postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::False
                    },
                    &select_only_created_or_updated_ids_query_bind_token_stream,
                )
            };
            let maybe_impl_postgresql_json_type_object_vec_element_id_for_ident_origin_token_stream = if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                let (query_bind_string_as_postgresql_text_create_for_query_token_stream, query_bind_string_as_postgresql_text_update_for_query_token_stream) = {
                    enum CreateForQueryOrUpdateForQuery {
                        CreateForQuery,
                        UpdateForQuery,
                    }
                    let generate_query_bind_string_as_postgresql_text_token_stream = |create_for_query_or_update_for_query: &CreateForQueryOrUpdateForQuery| {
                        let name_token_stream = format!(
                            "query_bind_string_as_postgresql_text_{}_for_query",
                            match &create_for_query_or_update_for_query {
                                CreateForQueryOrUpdateForQuery::CreateForQuery => "create",
                                CreateForQueryOrUpdateForQuery::UpdateForQuery => "update",
                            }
                        )
                        .parse::<proc_macro2::TokenStream>()
                        .expect("error f1bcde08-085f-4c98-9a1e-1fb583c9fb6e");
                        let type_token_stream: &dyn quote::ToTokens = match &create_for_query_or_update_for_query {
                            CreateForQueryOrUpdateForQuery::CreateForQuery => &create_for_query_upper_camel_case,
                            CreateForQueryOrUpdateForQuery::UpdateForQuery => &update_for_query_upper_camel_case,
                        };
                        quote::quote! {
                            fn #name_token_stream(
                                #value_snake_case: <Self::PostgresqlJsonType as #import_path::PostgresqlJsonType>::#type_token_stream,
                                mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
                            ) -> Result<
                                sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                                #std_string_string_token_stream
                            > {
                                if let Err(#error_snake_case) = #query_snake_case.try_bind(#value_snake_case.0.0.to_string()) {
                                    return Err(#error_snake_case.to_string())
                                }
                                Ok(#query_snake_case)
                            }
                        }
                    };
                    (generate_query_bind_string_as_postgresql_text_token_stream(&CreateForQueryOrUpdateForQuery::CreateForQuery), generate_query_bind_string_as_postgresql_text_token_stream(&CreateForQueryOrUpdateForQuery::UpdateForQuery))
                };
                quote::quote! {
                    impl #import_path::PostgresqlJsonTypeObjectVecElementId for #ident {
                        type PostgresqlJsonType = #ident;
                        type #create_for_query_upper_camel_case = #ident_create_for_query_upper_camel_case;
                        type #update_upper_camel_case = #ident_update_upper_camel_case;
                        type #read_inner_upper_camel_case = #ident_read_inner_upper_camel_case;
                        #query_bind_string_as_postgresql_text_create_for_query_token_stream
                        #query_bind_string_as_postgresql_text_update_for_query_token_stream
                        fn get_inner<'lifetime>(#value_snake_case: &'lifetime <Self::PostgresqlJsonType as #import_path::PostgresqlJsonType>::#create_for_query_upper_camel_case) -> &'lifetime Self::#read_inner_upper_camel_case {
                            &#value_snake_case.0.0
                        }
                        fn increment_checked_add_one(#increment_snake_case: &mut #std_primitive_u64_token_stream) -> Result<#std_primitive_u64_token_stream, #import_path::QueryPartErrorNamed> {
                            #import_path::increment_checked_add_one_returning_increment(#increment_snake_case)
                        }
                    }
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let impl_postgresql_json_type_test_cases_for_ident_token_stream = {
                let generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream = |read_or_update: &postgresql_crud_macros_common::ReadOrUpdate| {
                    let read_or_update_upper_camel_case = read_or_update.upper_camel_case();
                    quote::quote! {<#self_upper_camel_case::#postgresql_json_type_upper_camel_case
                        as
                        #postgresql_crud_macros_common_import_path_postgresql_crud_common::#postgresql_json_type_upper_camel_case
                    >::#read_or_update_upper_camel_case::#new_snake_case(#value_snake_case)}
                };
                let standart_not_null_test_cases_vec_name_token_stream = match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => quote::quote! {std_primitive_i8_test_cases_vec},
                    PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => quote::quote! {std_primitive_i16_test_cases_vec},
                    PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => quote::quote! {std_primitive_i32_test_cases_vec},
                    PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => quote::quote! {std_primitive_i64_test_cases_vec},
                    PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => quote::quote! {std_primitive_u8_test_cases_vec},
                    PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => quote::quote! {std_primitive_u16_test_cases_vec},
                    PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => quote::quote! {std_primitive_u32_test_cases_vec},
                    PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => quote::quote! {std_primitive_u64_test_cases_vec},
                    PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => quote::quote! {std_primitive_f32_test_cases_vec},
                    PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => quote::quote! {std_primitive_f64_test_cases_vec},
                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => quote::quote! {std_primitive_bool_test_cases_vec},
                    PostgresqlJsonType::StdStringStringAsJsonbString => quote::quote! {std_string_string_test_cases_vec},
                    PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {uuid_uuid_test_cases_vec},
                };
                let generate_array_dimension_equal_token_stream = |dimension: &postgresql_crud_macros_common::Dimension| {
                    use postgresql_crud_macros_common::NotNullOrNullable;
                    let dimension_index_number_max = postgresql_crud_macros_common::DimensionIndexNumber::from(dimension);
                    let index_number_to_std_primitive_u8 = |dimension_index_number: &postgresql_crud_macros_common::DimensionIndexNumber| -> u8 {
                        match dimension_index_number {
                            postgresql_crud_macros_common::DimensionIndexNumber::Zero => 0,
                            postgresql_crud_macros_common::DimensionIndexNumber::One => 1,
                            postgresql_crud_macros_common::DimensionIndexNumber::Two => 2,
                            postgresql_crud_macros_common::DimensionIndexNumber::Three => 3,
                        }
                    };
                    let generate_for_index_element_into_iter_enumerate_token_stream = |dimension_index_number: &postgresql_crud_macros_common::DimensionIndexNumber, in_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                        let index_number_token_stream = format!("index_{}", index_number_to_std_primitive_u8(dimension_index_number)).parse::<proc_macro2::TokenStream>().expect("error dd0a2fb8-40a5-4d63-95bc-f47a3656f652");
                        quote::quote! {
                            for (#index_number_token_stream, #value_snake_case) in #in_token_stream.into_iter().enumerate() {
                                #content_token_stream
                            }
                        }
                    };
                    let create_dot_zero_dot_zero_token_stream = quote::quote! {#create_snake_case.0.0};
                    let value_dot_zero_token_stream = quote::quote! {#value_snake_case.0};
                    let generate_maybe_if_some_token_stream = |current_not_null_or_nullable: &NotNullOrNullable, some_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| match current_not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote::quote! {#content_token_stream},
                        NotNullOrNullable::Nullable => quote::quote! {
                            if let Some(#value_snake_case) = #some_token_stream {
                                #content_token_stream
                            }
                        },
                    };
                    let generate_acc_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        let token_stream = generate_maybe_if_some_token_stream(not_null_or_nullable, &create_dot_zero_dot_zero_token_stream, &content_token_stream);
                        quote::quote! {
                            let mut #acc_snake_case = vec![];
                            #token_stream
                            if #acc_snake_case.is_empty() {
                                None
                            }
                            else {
                                Some(#acc_snake_case)
                            }
                        }
                    };
                    let generate_dimension_equal_initialization_token_stream = |current_value_ident_not_null_or_nullable: &NotNullOrNullable, current_value_ident_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
                        let to_number_starting_with_one_word_str = |dimension_index_number: &postgresql_crud_macros_common::DimensionIndexNumber| match dimension_index_number {
                            postgresql_crud_macros_common::DimensionIndexNumber::Zero => "One",
                            postgresql_crud_macros_common::DimensionIndexNumber::One => "Two",
                            postgresql_crud_macros_common::DimensionIndexNumber::Two => "Three",
                            postgresql_crud_macros_common::DimensionIndexNumber::Three => "Four",
                        };
                        let dimension_number_starting_with_one_equal_token_stream = format!("Dimension{}Equal", to_number_starting_with_one_word_str(&dimension_index_number_max)).parse::<proc_macro2::TokenStream>().expect("error 52fa34ac-5cd1-4ae9-8a1d-832e73a505d7");
                        let postgresql_json_type_where_dimension_number_starting_with_one_equal_token_stream = format!("PostgresqlJsonTypeWhereDimension{}Equal", to_number_starting_with_one_word_str(&dimension_index_number_max)).parse::<proc_macro2::TokenStream>().expect("error 15d769b0-0767-473c-a2c5-3d0f6e221ced");
                        let current_where_ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&generate_ident_token_stream(&NotNullOrNullable::NotNull, postgresql_json_type_pattern));
                        let current_value_ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(current_value_ident_not_null_or_nullable, current_value_ident_postgresql_json_type_pattern));
                        let vec_content_token_stream = {
                            let mut content_token_stream = vec![];
                            for current_element in 0..=index_number_to_std_primitive_u8(&dimension_index_number_max) {
                                let index_number_token_stream = format!("index_{current_element}").parse::<proc_macro2::TokenStream>().expect("error f0ce7e73-6d15-4de8-8f15-ce00334ed410");
                                content_token_stream.push(quote::quote! {
                                    postgresql_crud_common::UnsignedPartOfStdPrimitiveI32::try_from(
                                        i32::try_from(#index_number_token_stream).expect("error 5a1818e7-3865-4222-bf6b-31486bd721d2")
                                    ).expect("error ad1ab73f-fd3b-4162-adb0-bb09a19d31a0")
                                });
                            }
                            quote::quote! {#(#content_token_stream),*}
                        };
                        quote::quote! {
                            #current_where_ident_where_upper_camel_case::#dimension_number_starting_with_one_equal_token_stream(
                                where_filters::#postgresql_json_type_where_dimension_number_starting_with_one_equal_token_stream {
                                    logical_operator: #import_path::LogicalOperator::And,
                                    dimensions: where_filters::BoundedStdVecVec::try_from(
                                        vec![#vec_content_token_stream]
                                    ).expect("error 82cc0a3c-3e8d-47c4-b317-2795362a9b37"),
                                    #value_snake_case: #current_value_ident_table_type_declaration_upper_camel_case::new(#value_snake_case.into()),
                                }
                            )
                        }
                    };
                    let starting_value_token_stream = match not_null_or_nullable {
                        NotNullOrNullable::NotNull => &create_dot_zero_dot_zero_token_stream,
                        NotNullOrNullable::Nullable => &value_dot_zero_token_stream,
                    };
                    let generate_maybe_if_some_value_dot_zero_token_stream = |current_not_null_or_nullable: &NotNullOrNullable, content_token_stream: &dyn quote::ToTokens| generate_maybe_if_some_token_stream(
                        current_not_null_or_nullable,
                        &value_dot_zero_token_stream,
                        &content_token_stream
                    );
                    let generate_not_null_or_nullable_token_stream = |current_not_null_or_nullable: &NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
                        let content_token_stream = generate_dimension_equal_initialization_token_stream(current_not_null_or_nullable, current_postgresql_json_type_pattern);
                        match not_null_or_nullable {
                            NotNullOrNullable::NotNull => quote::quote! {#acc_snake_case.push(#content_token_stream);},
                            NotNullOrNullable::Nullable => quote::quote! {
                                if let Ok(#value_snake_case) = #import_path::NotEmptyUniqueEnumVec::try_new(vec![#content_token_stream]) {
                                    #acc_snake_case.push(#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(
                                        #value_snake_case
                                    )));
                                }
                            },
                        }
                    };
                    let generate_for_index_element_into_iter_enumerate_three_token_stream = |in_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| generate_for_index_element_into_iter_enumerate_token_stream(&postgresql_crud_macros_common::DimensionIndexNumber::Three, &in_token_stream, &content_token_stream);
                    let generate_for_index_element_into_iter_enumerate_two_token_stream = |in_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| generate_for_index_element_into_iter_enumerate_token_stream(&postgresql_crud_macros_common::DimensionIndexNumber::Two, &in_token_stream, &content_token_stream);
                    let generate_for_index_element_into_iter_enumerate_one_token_stream = |in_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| generate_for_index_element_into_iter_enumerate_token_stream(&postgresql_crud_macros_common::DimensionIndexNumber::One, &in_token_stream, &content_token_stream);
                    let generate_for_index_element_into_iter_enumerate_zero_token_stream = |in_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| generate_for_index_element_into_iter_enumerate_token_stream(&postgresql_crud_macros_common::DimensionIndexNumber::Zero, &in_token_stream, &content_token_stream);
                    let generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_for_index_element_into_iter_enumerate_zero_token_stream(&starting_value_token_stream, &content_token_stream);
                    let generate_for_maybe_if_some_token_stream = |dimension_index_number: &postgresql_crud_macros_common::DimensionIndexNumber, current_not_null_or_nullable: &NotNullOrNullable, content_token_stream: &dyn quote::ToTokens| {
                        generate_maybe_if_some_value_dot_zero_token_stream(
                            current_not_null_or_nullable,
                            &generate_for_index_element_into_iter_enumerate_token_stream(dimension_index_number, &value_dot_zero_token_stream, &content_token_stream)
                        )
                    };
                    let generate_down_postgresql_json_type_pattern = || match dimension_index_number_max {
                        postgresql_crud_macros_common::DimensionIndexNumber::Zero => postgresql_json_type_pattern.down_by_1(),
                        postgresql_crud_macros_common::DimensionIndexNumber::One => postgresql_json_type_pattern.down_by_2(),
                        postgresql_crud_macros_common::DimensionIndexNumber::Two => postgresql_json_type_pattern.down_by_3(),
                        postgresql_crud_macros_common::DimensionIndexNumber::Three => postgresql_json_type_pattern.down_by_4(),
                    };
                    match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => none_token_stream.clone(),
                        PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match dimension_index_number_max {
                            postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_acc_token_stream(&{
                                let current_postgresql_json_type_pattern = generate_down_postgresql_json_type_pattern().expect("error 63f3476d-e0e0-471c-9faa-0a626c8ba75e");
                                let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(&generate_not_null_or_nullable_token_stream(dimension1_not_null_or_nullable, &current_postgresql_json_type_pattern));
                                quote::quote! {#dimension1_token_stream}
                            }),
                            postgresql_crud_macros_common::DimensionIndexNumber::One | postgresql_crud_macros_common::DimensionIndexNumber::Two | postgresql_crud_macros_common::DimensionIndexNumber::Three => none_token_stream.clone(),
                        },
                        PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => match dimension_index_number_max {
                            postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_acc_token_stream(&{
                                let current_postgresql_json_type_pattern = generate_down_postgresql_json_type_pattern().expect("error 99c97e51-792f-40e7-bdfe-f7424803e368");
                                let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(&generate_not_null_or_nullable_token_stream(dimension1_not_null_or_nullable, &current_postgresql_json_type_pattern));
                                quote::quote! {#dimension1_token_stream}
                            }),
                            postgresql_crud_macros_common::DimensionIndexNumber::One => generate_acc_token_stream(&{
                                let current_postgresql_json_type_pattern = generate_down_postgresql_json_type_pattern().expect("error 23f9b122-9788-4673-b996-5d437b363f7e");
                                let dimension2_token_stream = generate_for_index_element_into_iter_enumerate_one_token_stream(&value_dot_zero_token_stream, &generate_not_null_or_nullable_token_stream(dimension2_not_null_or_nullable, &current_postgresql_json_type_pattern));
                                let maybe_if_some_dimension2_token_stream = generate_maybe_if_some_value_dot_zero_token_stream(dimension1_not_null_or_nullable, &dimension2_token_stream);
                                let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(&maybe_if_some_dimension2_token_stream);
                                quote::quote! {#dimension1_token_stream}
                            }),
                            postgresql_crud_macros_common::DimensionIndexNumber::Two | postgresql_crud_macros_common::DimensionIndexNumber::Three => none_token_stream.clone(),
                        },
                        PostgresqlJsonTypePattern::ArrayDimension3 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                        } => match dimension_index_number_max {
                            postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_acc_token_stream(&{
                                let current_postgresql_json_type_pattern = generate_down_postgresql_json_type_pattern().expect("error 6b26e2ac-4462-451d-9111-01f659357a41");
                                let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(&generate_not_null_or_nullable_token_stream(dimension1_not_null_or_nullable, &current_postgresql_json_type_pattern));
                                quote::quote! {#dimension1_token_stream}
                            }),
                            postgresql_crud_macros_common::DimensionIndexNumber::One => generate_acc_token_stream(&{
                                let current_postgresql_json_type_pattern = generate_down_postgresql_json_type_pattern().expect("error 66e824f8-7635-4f3d-8fcc-cdd629189cfe");
                                let dimension2_token_stream = generate_for_index_element_into_iter_enumerate_one_token_stream(&value_dot_zero_token_stream, &generate_not_null_or_nullable_token_stream(dimension2_not_null_or_nullable, &current_postgresql_json_type_pattern));
                                let maybe_if_some_dimension2_token_stream = generate_maybe_if_some_value_dot_zero_token_stream(dimension1_not_null_or_nullable, &dimension2_token_stream);
                                let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(&maybe_if_some_dimension2_token_stream);
                                quote::quote! {#dimension1_token_stream}
                            }),
                            postgresql_crud_macros_common::DimensionIndexNumber::Two => generate_acc_token_stream(&{
                                let current_postgresql_json_type_pattern = generate_down_postgresql_json_type_pattern().expect("error 55896a34-0056-48f1-b79b-69391daa149a");
                                let dimension3_token_stream = generate_for_index_element_into_iter_enumerate_two_token_stream(&value_dot_zero_token_stream, &generate_not_null_or_nullable_token_stream(dimension3_not_null_or_nullable, &current_postgresql_json_type_pattern));
                                let maybe_if_some_dimension3_token_stream = generate_maybe_if_some_value_dot_zero_token_stream(dimension2_not_null_or_nullable, &dimension3_token_stream);
                                let maybe_if_some_dimension2_token_stream = generate_for_maybe_if_some_token_stream(&postgresql_crud_macros_common::DimensionIndexNumber::One, dimension1_not_null_or_nullable, &maybe_if_some_dimension3_token_stream);
                                let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(&maybe_if_some_dimension2_token_stream);
                                quote::quote! {#dimension1_token_stream}
                            }),
                            postgresql_crud_macros_common::DimensionIndexNumber::Three => none_token_stream.clone(),
                        },
                        PostgresqlJsonTypePattern::ArrayDimension4 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                            dimension4_not_null_or_nullable,
                        } => {
                            let current_postgresql_json_type_pattern = generate_down_postgresql_json_type_pattern().expect("error 9048d6b1-5312-4c91-b48f-7f2adb197135");
                            match dimension_index_number_max {
                                postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_acc_token_stream(&{
                                    let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(&generate_not_null_or_nullable_token_stream(dimension1_not_null_or_nullable, &current_postgresql_json_type_pattern));
                                    quote::quote! {#dimension1_token_stream}
                                }),
                                postgresql_crud_macros_common::DimensionIndexNumber::One => generate_acc_token_stream(&{
                                    let dimension2_token_stream = generate_for_index_element_into_iter_enumerate_one_token_stream(&value_dot_zero_token_stream, &generate_not_null_or_nullable_token_stream(dimension2_not_null_or_nullable, &current_postgresql_json_type_pattern));
                                    let maybe_if_some_dimension2_token_stream = generate_maybe_if_some_value_dot_zero_token_stream(dimension1_not_null_or_nullable, &dimension2_token_stream);
                                    let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(&maybe_if_some_dimension2_token_stream);
                                    quote::quote! {#dimension1_token_stream}
                                }),
                                postgresql_crud_macros_common::DimensionIndexNumber::Two => generate_acc_token_stream(&{
                                    let dimension3_token_stream = generate_for_index_element_into_iter_enumerate_two_token_stream(&value_dot_zero_token_stream, &generate_not_null_or_nullable_token_stream(dimension3_not_null_or_nullable, &current_postgresql_json_type_pattern));
                                    let maybe_if_some_dimension3_token_stream = generate_maybe_if_some_value_dot_zero_token_stream(dimension2_not_null_or_nullable, &dimension3_token_stream);
                                    let maybe_if_some_dimension2_token_stream = generate_for_maybe_if_some_token_stream(&postgresql_crud_macros_common::DimensionIndexNumber::One, dimension1_not_null_or_nullable, &maybe_if_some_dimension3_token_stream);
                                    let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(&maybe_if_some_dimension2_token_stream);
                                    quote::quote! {#dimension1_token_stream}
                                }),
                                postgresql_crud_macros_common::DimensionIndexNumber::Three => generate_acc_token_stream(&{
                                    let dimension4_token_stream = generate_for_index_element_into_iter_enumerate_three_token_stream(&value_dot_zero_token_stream, &generate_not_null_or_nullable_token_stream(dimension4_not_null_or_nullable, &current_postgresql_json_type_pattern));
                                    let maybe_if_some_dimension4_token_stream = generate_maybe_if_some_value_dot_zero_token_stream(dimension3_not_null_or_nullable, &dimension4_token_stream);
                                    let maybe_if_some_dimension3_token_stream = generate_for_maybe_if_some_token_stream(&postgresql_crud_macros_common::DimensionIndexNumber::Two, dimension2_not_null_or_nullable, &maybe_if_some_dimension4_token_stream);
                                    let maybe_if_some_dimension2_token_stream = generate_for_maybe_if_some_token_stream(&postgresql_crud_macros_common::DimensionIndexNumber::One, dimension1_not_null_or_nullable, &maybe_if_some_dimension3_token_stream);
                                    let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(&maybe_if_some_dimension2_token_stream);
                                    quote::quote! {#dimension1_token_stream}
                                }),
                            }
                        }
                    }
                };
                let option_vec_create_token_stream = {
                    use postgresql_crud_macros_common::NotNullOrNullable;
                    let generate_some_acc_content_token_stream = |current_not_null_or_nullable: &NotNullOrNullable, current_ident_token_stream: &dyn quote::ToTokens| {
                        let (new_content_token_stream, maybe_acc_push_none_token_stream) = match &current_not_null_or_nullable {
                            NotNullOrNullable::NotNull => (quote::quote! {vec![#element_snake_case.0.into()]}, proc_macro2::TokenStream::new()),
                            NotNullOrNullable::Nullable => (quote::quote! {Some(#element_snake_case.0.into())}, quote::quote! {#acc_snake_case.push(<#ident as #import_path::PostgresqlJsonType>::Create::new(None));}),
                        };
                        //todo check - maybe need to add something here
                        let maybe_acc_push_long_vec_token_stream = match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => quote::quote! {
                                if let Some(#value_snake_case) = <#current_ident_token_stream as #import_path::PostgresqlJsonTypeTestCases>::#option_vec_create_snake_case() {
                                    let mut inner_acc = vec![];
                                    for #element_snake_case in #value_snake_case {
                                        inner_acc.push(#element_snake_case.0.into());
                                    }
                                    if !inner_acc.is_empty() {
                                        #acc_snake_case.push(<#ident as #import_path::PostgresqlJsonType>::Create::new(inner_acc));
                                    }
                                }
                            },
                            NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                        };
                        quote::quote! {Some({
                            let mut #acc_snake_case = vec![];
                            if let Some(#value_snake_case) = <#current_ident_token_stream as #import_path::PostgresqlJsonTypeTestCases>::#option_vec_create_snake_case() {
                                for #element_snake_case in #value_snake_case {
                                    #acc_snake_case.push(<#ident as #import_path::PostgresqlJsonType>::Create::new(#new_content_token_stream));
                                }
                            }
                            #maybe_acc_push_long_vec_token_stream
                            #maybe_acc_push_none_token_stream
                            #acc_snake_case
                        })}
                    };
                    let content_token_stream = match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => quote::quote! {
                                Some(
                                    #import_path::#standart_not_null_test_cases_vec_name_token_stream().into_iter().map(|#element_snake_case|
                                        <#ident as #import_path::PostgresqlJsonType>::Create::new(#element_snake_case)
                                    ).collect()
                                )
                            },
                            NotNullOrNullable::Nullable => generate_some_acc_content_token_stream(not_null_or_nullable, &generate_ident_token_stream(&NotNullOrNullable::NotNull, &PostgresqlJsonTypePattern::Standart)),
                        },
                        PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_some_acc_content_token_stream(
                            not_null_or_nullable,
                            &match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error dec468c0-09fd-4db3-98e7-7fa9cd565909")),
                                NotNullOrNullable::Nullable => generate_ident_token_stream(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                            },
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_some_acc_content_token_stream(
                            not_null_or_nullable,
                            &match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 4010ebf7-d6e2-4d6e-a562-a299201c92ec")),
                                NotNullOrNullable::Nullable => generate_ident_token_stream(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                            },
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_some_acc_content_token_stream(
                            not_null_or_nullable,
                            &match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error acdbb564-b169-40db-9815-2653c0150a4c")),
                                NotNullOrNullable::Nullable => generate_ident_token_stream(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                            },
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_some_acc_content_token_stream(
                            not_null_or_nullable,
                            &match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 5abf9504-cde0-4c6c-adb9-145b385918a5")),
                                NotNullOrNullable::Nullable => generate_ident_token_stream(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                            },
                        ),
                    };
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
                        | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                        | PostgresqlJsonType::StdStringStringAsJsonbString => content_token_stream,
                        PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {None},
                    }
                };
                let read_only_ids_to_two_dimensional_vec_read_inner_token_stream = {
                    use postgresql_crud_macros_common::NotNullOrNullable;
                    let (has_len_greater_than_one_token_stream, has_len_greater_than_one_for_for_token_stream) = {
                        let generate_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {let has_len_greater_than_one = #content_token_stream;}
                        };
                        (
                            generate_token_stream(&quote::quote! {read_only_ids_to_two_dimensional_vec_read_inner.len() > 1}),
                            generate_token_stream(&quote::quote! {{
                                let mut has_len_greater_than_one = false;
                                for #element_snake_case in &read_only_ids_to_two_dimensional_vec_read_inner {
                                    if #element_snake_case.len() > 1 {
                                        has_len_greater_than_one = true;
                                        break;
                                    }
                                }
                                has_len_greater_than_one
                            }}),
                        )
                    };
                    let generate_acc_content_handle_token_stream = |current_ident_token_stream: &dyn quote::ToTokens, has_len_greater_than_one_content_token_stream: &dyn quote::ToTokens| {
                        let current_ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&current_ident_token_stream);
                        let option_additional_content_token_stream = {
                            let element1_clone_token_stream = quote::quote! {element1.clone()};
                            let first = quote::quote! {vec![#element1_clone_token_stream]};
                            let second = quote::quote! {vec![#element1_clone_token_stream, #element1_clone_token_stream]};
                            let (first_content_token_stream, second_content_token_stream) = match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => (first, second),
                                NotNullOrNullable::Nullable => {
                                    let generate_token_stream = |content_token_stream: &dyn quote::ToTokens| quote::quote! {Some(#content_token_stream)};
                                    (generate_token_stream(&first), generate_token_stream(&second))
                                }
                            };
                            quote::quote! {
                                let option_additional = {
                                    let mut option_additional = None;
                                    for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                        if option_additional.is_some() {
                                            break;
                                        }
                                        for element1 in element0 {
                                            if option_additional.is_none() {
                                                option_additional = Some((vec![#first_content_token_stream], vec![#second_content_token_stream]));
                                            }
                                            else {
                                                break;
                                            }
                                        }
                                    }
                                    option_additional
                                };
                            }
                        };
                        let acc_push_vec_content_token_stream = {
                            let content_token_stream = {
                                let inner_content_token_stream = quote::quote! {{
                                    let mut #acc_snake_case = vec![];
                                    for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                                        for element1 in element0 {
                                            #acc_snake_case.push(element1);
                                        }
                                    }
                                    #acc_snake_case
                                }};
                                match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => inner_content_token_stream,
                                    NotNullOrNullable::Nullable => quote::quote! {Some(#inner_content_token_stream)},
                                }
                            };
                            quote::quote! {#acc_snake_case.push(vec![#content_token_stream]);}
                        };
                        let maybe_acc_push_vec_none_token_stream = match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => proc_macro2::TokenStream::new(),
                            NotNullOrNullable::Nullable => quote::quote! {#acc_snake_case.push(vec![None]);},
                        };
                        quote::quote! {
                            let mut #acc_snake_case = vec![];
                            let read_only_ids_to_two_dimensional_vec_read_inner = <#current_ident_token_stream as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(&#current_ident_read_only_ids_upper_camel_case(read_only_ids.0.clone()));
                            #option_additional_content_token_stream
                            #has_len_greater_than_one_content_token_stream
                            #acc_push_vec_content_token_stream
                            #maybe_acc_push_vec_none_token_stream
                            if let Some(#value_snake_case) = option_additional {
                                if has_len_greater_than_one {
                                    #acc_snake_case.push(#value_snake_case.0);
                                }
                                if !has_len_greater_than_one {
                                    #acc_snake_case.push(#value_snake_case.1);
                                }
                            }
                            #acc_snake_case
                        }
                    };
                    let content_token_stream = match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => quote::quote! {vec![#import_path::#standart_not_null_test_cases_vec_name_token_stream().into()]},
                            NotNullOrNullable::Nullable => quote::quote! {
                                let mut #acc_snake_case = vec![];
                                for element0 in <#ident_standart_not_null_upper_camel_case as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(&#ident_read_only_ids_standart_not_null_upper_camel_case(read_only_ids.0.clone())) {
                                    for element1 in element0 {
                                        #acc_snake_case.push(vec![Some(element1)]);
                                    }
                                }
                                #acc_snake_case.push(vec![None]);
                                #acc_snake_case
                            },
                        },
                        PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_acc_content_handle_token_stream(
                            &generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error d6f89137-9a47-4f74-afce-0e1959d3dc59")),
                            &match &dimension1_not_null_or_nullable {
                                NotNullOrNullable::NotNull => &has_len_greater_than_one_for_for_token_stream,
                                NotNullOrNullable::Nullable => &has_len_greater_than_one_token_stream,
                            },
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_acc_content_handle_token_stream(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 38774398-d485-4c14-84e8-92e06c36c23b")), &has_len_greater_than_one_token_stream),
                        PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_acc_content_handle_token_stream(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 053f4bab-0a8e-457f-9176-50e519b312bb")), &has_len_greater_than_one_token_stream),
                        PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_acc_content_handle_token_stream(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("error 860f8f15-72ac-4557-a2c6-87b1aa958eb4")), &has_len_greater_than_one_token_stream),
                    };
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
                        | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                        | PostgresqlJsonType::StdStringStringAsJsonbString => content_token_stream,
                        PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {vec![]},
                    }
                };
                let read_inner_into_read_with_new_or_try_new_unwraped_token_stream = generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(&postgresql_crud_macros_common::ReadOrUpdate::Read);
                let read_inner_into_update_with_new_or_try_new_unwraped_token_stream = generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(&postgresql_crud_macros_common::ReadOrUpdate::Update);
                let read_only_ids_into_option_value_read_inner_token_stream = {
                    let content_token_stream = generate_import_path_value_initialization_token_stream(&if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                        quote::quote! {#value_snake_case.0.#value_snake_case}
                    } else {
                        quote::quote! {
                            <#ident as #import_path::PostgresqlJsonType>::into_inner(
                                <
                                    <#ident as #import_path::PostgresqlJsonType>::Read
                                    as
                                    #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                                >::default_but_option_is_always_some_and_vec_always_contains_one_element()
                            )
                        }
                    });
                    quote::quote! {Some(#content_token_stream)}
                };
                let update_to_read_only_ids_token_stream = {
                    let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                        let generate_update_to_read_only_ids_token_stream = |current_ident_token_stream: &dyn quote::ToTokens, current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable| {
                            let current_ident_update_token_stream = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&current_ident_token_stream);
                            let content_token_stream = {
                                let content_token_stream = match &current_not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#element_snake_case.clone()},
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {#value_snake_case.clone().into()},
                                };
                                quote::quote! {&#current_ident_update_token_stream(#content_token_stream)}
                            };
                            quote::quote! {
                                <
                                    #current_ident_token_stream
                                    as
                                    #import_path::PostgresqlJsonTypeTestCases
                                >::update_to_read_only_ids(&#content_token_stream).0.#value_snake_case
                            }
                        };
                        let generate_iter_or_match_token_stream = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, content_token_stream: &dyn quote::ToTokens| {
                            let value_zero_zero_token_stream = quote::quote! {#value_snake_case.0.0};
                            match &current_not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                    #value_zero_zero_token_stream.iter().map(|#element_snake_case|#content_token_stream).collect()
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                    match &#value_zero_zero_token_stream {
                                        Some(#value_snake_case) => Some(#content_token_stream),
                                        None => None
                                    }
                                },
                            }
                        };
                        match &postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#value_snake_case.0.clone().into()},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => 
                                generate_iter_or_match_token_stream(
                                    not_null_or_nullable,
                                    &generate_update_to_read_only_ids_token_stream(
                                        &ident_not_null_token_stream,
                                        not_null_or_nullable
                                    )
                                ),
                            },
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_iter_or_match_token_stream(
                                not_null_or_nullable,
                                &generate_update_to_read_only_ids_token_stream(
                                    &generate_ident_token_stream(
                                        &match &not_null_or_nullable {
                                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                        },
                                        &match &not_null_or_nullable {
                                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("error e84064c3-5c31-4fa6-8dbc-ba454128c9da"),
                                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                        },
                                    ),
                                    not_null_or_nullable,
                                ),
                            ),
                            PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_iter_or_match_token_stream(
                                not_null_or_nullable,
                                &generate_update_to_read_only_ids_token_stream(
                                    &generate_ident_token_stream(
                                        &match &not_null_or_nullable {
                                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                        },
                                        &match &not_null_or_nullable {
                                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("error 226c6318-6be3-4b85-b2cd-c0b53a2d6bf9"),
                                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                        },
                                    ),
                                    not_null_or_nullable,
                                ),
                            ),
                            PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_iter_or_match_token_stream(
                                not_null_or_nullable,
                                &generate_update_to_read_only_ids_token_stream(
                                    &generate_ident_token_stream(
                                        &match &not_null_or_nullable {
                                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                        },
                                        &match &not_null_or_nullable {
                                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("error 3ae1e9f8-84ec-4369-a633-eca188d9b10a"),
                                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                        },
                                    ),
                                    not_null_or_nullable,
                                ),
                            ),
                            PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_iter_or_match_token_stream(
                                not_null_or_nullable,
                                &generate_update_to_read_only_ids_token_stream(
                                    &generate_ident_token_stream(
                                        &match &not_null_or_nullable {
                                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                        },
                                        &match &not_null_or_nullable {
                                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("error 44d51dc5-1b15-4807-ad63-c4fcfb01251c"),
                                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                        },
                                    ),
                                    not_null_or_nullable,
                                ),
                            ),
                        }
                    } else {
                        none_token_stream.clone()
                    });
                    quote::quote! {#ident_read_only_ids_upper_camel_case(#value_initialization_token_stream)}
                };
                let read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = {
                    let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                        quote::quote! {#ident_read_upper_camel_case::new(#value_snake_case.0.#value_snake_case.clone())}
                    } else {
                        quote::quote! {#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}
                    });
                    quote::quote! {Some(#value_initialization_token_stream)}
                };
                let previous_read_merged_with_option_update_into_read_token_stream = quote::quote! {match #option_update_snake_case {
                    Some(#value_snake_case) => #ident_read_upper_camel_case(#value_snake_case.into()),
                    None => #read_snake_case
                }};
                let read_only_ids_merged_with_create_into_read_token_stream = {
                    let content_token_stream = if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                        quote::quote! {#ident_origin_upper_camel_case::new(#read_only_ids_snake_case.0.#value_snake_case)}
                    } else {
                        quote::quote! {#create_snake_case.into()}
                    };
                    quote::quote! {#ident_read_upper_camel_case(#content_token_stream)}
                };
                let read_only_ids_merged_with_create_into_option_value_read_token_stream = {
                    let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&quote::quote! {
                        <#ident as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_read_snake_case(
                            #read_only_ids_snake_case,
                            #create_snake_case
                        )
                    });
                    quote::quote! {Some(#value_initialization_token_stream)}
                };
                let read_only_ids_merged_with_create_into_table_type_declaration_token_stream = {
                    let content_token_stream = if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                        quote::quote! {#ident_origin_upper_camel_case::new(#read_only_ids_snake_case.0.#value_snake_case)}
                    } else {
                        quote::quote! {#create_snake_case.into()}
                    };
                    quote::quote! {#ident_table_type_declaration_upper_camel_case(#content_token_stream)}
                };
                let read_only_ids_merged_with_create_into_where_equal_token_stream = {
                    let generate_equal_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            where_filters::PostgresqlJsonTypeWhereEqual {
                                logical_operator: #import_path::LogicalOperator::Or,
                                #value_snake_case: #content_token_stream
                            }
                        }
                    };
                    match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                            let equal_token_stream = generate_equal_token_stream(&quote::quote! {#ident_table_type_declaration_upper_camel_case::new(#create_snake_case.0.into())});
                            quote::quote! {#ident_where_upper_camel_case::#equal_upper_camel_case(#equal_token_stream)}
                        }
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let current_ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident_not_null_token_stream);
                            let current_ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident_not_null_token_stream);
                            let equal_token_stream = generate_equal_token_stream(&quote::quote! {#current_ident_table_type_declaration_upper_camel_case::new(#value_snake_case.into())});
                            quote::quote! {
                                #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(match #create_snake_case.0.0 {
                                    Some(#value_snake_case) => Some(
                                        #import_path::NotEmptyUniqueEnumVec::try_new(
                                            vec![#current_ident_where_upper_camel_case::#equal_upper_camel_case(#equal_token_stream)]
                                        ).expect("error 88bfa095-a3ab-4d0c-be71-af63c3acd50f")
                                    ),
                                    None => None,
                                })
                            }
                        }
                    }
                };
                let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream = quote::quote! {vec![
                    <#ident as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_where_equal_snake_case(
                        #read_only_ids_snake_case,
                        #create_snake_case
                    )
                ]};
                let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream = quote::quote! {<#ident as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case(
                    #read_only_ids_snake_case,
                    #create_snake_case
                )};
                let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream = generate_array_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::One);
                let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream = generate_array_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Two);
                let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream = generate_array_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Three);
                let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream = generate_array_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Four);
                //todo maybe reuse LengthEqual and LengthGreaterThan
                let create_into_postgresql_json_type_option_vec_where_length_equal_token_stream = {
                    let generate_token_stream = || {
                        let content_token_stream = {
                            let create_dot_zero_dot_zero = quote::quote! {#create_snake_case.0.0};
                            let content_token_stream = {
                                let content_token_stream: &dyn quote::ToTokens = match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &create_dot_zero_dot_zero,
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &quote::quote! {#value_snake_case.0},
                                };
                                quote::quote! {
                                    ::LengthEqual(
                                        where_filters::PostgresqlJsonTypeWhereLengthEqual {
                                            logical_operator: #import_path::LogicalOperator::Or,
                                            #value_snake_case: postgresql_crud_common::UnsignedPartOfStdPrimitiveI32::try_from(
                                                i32::try_from(#content_token_stream.len()).expect("error 56aee101-8823-4a80-bb06-c77ce1955151")
                                            ).expect("error aa5ac3cd-ad8a-4e90-af21-ad583792bc36"),
                                        }
                                    )
                                }
                            };
                            match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#ident_where_upper_camel_case #content_token_stream},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let current_ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident_not_null_token_stream);
                                    quote::quote! {
                                        #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(match #create_dot_zero_dot_zero {
                                            Some(#value_snake_case) => Some(
                                                #import_path::NotEmptyUniqueEnumVec::try_new(
                                                    vec![#current_ident_where_upper_camel_case #content_token_stream]
                                                ).expect("error cb1c6535-8b63-4756-a7b3-cab5b21de2d7")
                                            ),
                                            None => None,
                                        })
                                    }
                                }
                            }
                        };
                        quote::quote! {Some(vec![#content_token_stream])}
                    };
                    match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => none_token_stream.clone(),
                        PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => generate_token_stream(),
                    }
                };
                let postgresql_json_type_option_vec_where_length_greater_than_test_token_stream = {
                    // use postgresql_crud_macros_common::NotNullOrNullable;
                    // match &postgresql_json_type_pattern {
                    //     PostgresqlJsonTypePattern::Standart => quote::quote! {None},
                    //     PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                    //         (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) |
                    //         (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) |
                    //         (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) |
                    //         (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote! {todo!()},
                    //     },
                    //     PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable: _, dimension2_not_null_or_nullable: _ } => quote::quote! {todo!()},
                    //     PostgresqlJsonTypePattern::ArrayDimension3 {
                    //         dimension1_not_null_or_nullable: _,
                    //         dimension2_not_null_or_nullable: _,
                    //         dimension3_not_null_or_nullable: _,
                    //     } => quote::quote! {todo!()},
                    //     PostgresqlJsonTypePattern::ArrayDimension4 {
                    //         dimension1_not_null_or_nullable: _,
                    //         dimension2_not_null_or_nullable: _,
                    //         dimension3_not_null_or_nullable: _,
                    //         dimension4_not_null_or_nullable: _,
                    //     } => quote::quote! {todo!()}
                    // }
                    quote::quote! {todo!()}
                };
                let create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream = {
                    let generate_token_stream = || {
                        let content_token_stream = {
                            let create_dot_zero_dot_zero = quote::quote! {#create_snake_case.0.0};
                            let content_token_stream = {
                                let content_token_stream: &dyn quote::ToTokens = match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &create_dot_zero_dot_zero,
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &quote::quote! {#value_snake_case.0},
                                };
                                quote::quote! {
                                    ::LengthGreaterThan(
                                        where_filters::PostgresqlJsonTypeWhereLengthGreaterThan {
                                            logical_operator: #import_path::LogicalOperator::Or,
                                            #value_snake_case: postgresql_crud_common::UnsignedPartOfStdPrimitiveI32::try_from(
                                                i32::try_from(#content_token_stream.len()).expect("error 56aee101-8823-4a80-bb06-c77ce1955151")
                                            ).expect("error aa5ac3cd-ad8a-4e90-af21-ad583792bc36"),
                                        }
                                    )
                                }
                            };
                            match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#ident_where_upper_camel_case #content_token_stream},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let current_ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident_not_null_token_stream);
                                    quote::quote! {
                                        #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(match #create_dot_zero_dot_zero {
                                            Some(#value_snake_case) => Some(
                                                #import_path::NotEmptyUniqueEnumVec::try_new(
                                                    vec![#current_ident_where_upper_camel_case #content_token_stream]
                                                ).expect("error cb1c6535-8b63-4756-a7b3-cab5b21de2d7")
                                            ),
                                            None => None,
                                        })
                                    }
                                }
                            }
                        };
                        quote::quote! {Some(vec![#content_token_stream])}
                    };
                    match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => none_token_stream.clone(),
                        PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => generate_token_stream(),
                    }
                };
                postgresql_crud_macros_common::generate_impl_postgresql_json_type_test_cases_for_ident_token_stream(
                    &quote::quote! {#[cfg(feature = "test-utils")]},
                    &postgresql_crud_macros_common_import_path_postgresql_crud_common,
                    &ident_read_inner_upper_camel_case,
                    &ident,
                    &option_vec_create_token_stream,
                    &read_only_ids_to_two_dimensional_vec_read_inner_token_stream,
                    &read_inner_into_read_with_new_or_try_new_unwraped_token_stream,
                    &read_inner_into_update_with_new_or_try_new_unwraped_token_stream,
                    &read_only_ids_into_option_value_read_inner_token_stream,
                    &update_to_read_only_ids_token_stream,
                    &read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                    &previous_read_merged_with_option_update_into_read_token_stream,
                    &read_only_ids_merged_with_create_into_read_token_stream,
                    &read_only_ids_merged_with_create_into_option_value_read_token_stream,
                    &read_only_ids_merged_with_create_into_table_type_declaration_token_stream,
                    &read_only_ids_merged_with_create_into_where_equal_token_stream,
                    &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream,
                    &read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream,
                    &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream,
                    &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream,
                    &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream,
                    &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream,
                    &create_into_postgresql_json_type_option_vec_where_length_equal_token_stream,
                    &postgresql_json_type_option_vec_where_length_greater_than_test_token_stream,
                    &create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream,
                )
            };
            let generated = quote::quote! {
                #ident_token_stream
                #ident_origin_token_stream
                #ident_table_type_declaration_token_stream
                #ident_create_token_stream
                #ident_create_for_query_token_stream
                #ident_select_token_stream
                #ident_where_token_stream
                #ident_read_token_stream
                #ident_read_only_ids_token_stream
                #ident_read_inner_token_stream
                #ident_update_token_stream
                #ident_update_for_query_token_stream
                #impl_postgresql_json_type_for_ident_token_stream
                #maybe_impl_postgresql_json_type_object_vec_element_id_for_ident_origin_token_stream
                #impl_postgresql_json_type_test_cases_for_ident_token_stream
            };
            (
                {
                    let field_ident = format!("field_{index}").parse::<proc_macro2::TokenStream>().expect("error f992f797-a4df-40d0-9984-3a3a3ad439d7");
                    quote::quote! {
                        pub #field_ident: #ident,
                    }
                    .to_string()
                },
                generated.to_string(),
            )
        })
        .collect::<(Vec<String>, Vec<String>)>();
    // let example_token_stream = {
    //     let fields_token_stream = fields_token_stream
    //         .into_iter()
    //         .map(|element| element.parse::<proc_macro2::TokenStream>().expect("error 1d8cd8e4-5f51-4aed-a626-79d759d86ebf"))
    //         .collect::<Vec<proc_macro2::TokenStream>>();
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
    //         &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
    //     );
    // }
    let generated = {
        let content_token_stream = postgresql_json_type_array.into_iter().map(|element| element.parse::<proc_macro2::TokenStream>().expect("error 84e21b40-b5a4-4f4c-86d3-8f6ecfbe1f6e")).collect::<Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            #(#content_token_stream)*
            // #example_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlJsonTypes",
    //     &generated,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
    // );
    generated.into()
}
