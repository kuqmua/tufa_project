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
            let value: &dyn std::fmt::Display = match &self {
                Self::Number => &naming::NumberUpperCamelCase,
                Self::Boolean => &naming::BooleanUpperCamelCase,
                Self::String => &naming::StringUpperCamelCase,
            };
            write!(formatter, "{}{value}", naming::JsonbUpperCamelCase)
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
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
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
            self.to_string().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonType to proc_macro2::TokenStream")).to_tokens(tokens)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
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
                PostgresqlJsonTypePattern::ArrayDimension2 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable: _,
                } => Ok(Self::ArrayDimension2 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                }),
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
                Self::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                } => vec![&dimension2_not_null_or_nullable, &dimension1_not_null_or_nullable],
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
                ArrayDimension::ArrayDimension2 { dimension1_not_null_or_nullable } => Ok(Self::ArrayDimension2 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                }),
                ArrayDimension::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                } => Ok(Self::ArrayDimension3 {
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
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
                                    not_null_or_nullable: not_null_or_nullable.clone(),
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
                                        not_null_or_nullable: not_null_or_nullable.clone(),
                                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension2 {
                                            dimension1_not_null_or_nullable: dimension1_not_null_or_nullable.clone(),
                                            dimension2_not_null_or_nullable,
                                        },
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
                                            not_null_or_nullable: not_null_or_nullable.clone(),
                                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension3 {
                                                dimension1_not_null_or_nullable: dimension1_not_null_or_nullable.clone(),
                                                dimension2_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
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
                                                not_null_or_nullable: not_null_or_nullable.clone(),
                                                postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension4 {
                                                    dimension1_not_null_or_nullable: dimension1_not_null_or_nullable.clone(),
                                                    dimension2_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                                                    dimension3_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                                                    dimension4_not_null_or_nullable: dimension4_not_null_or_nullable,
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
    let postgresql_json_type_record_vec = {
        if false {
            PostgresqlJsonTypeRecord::all()
        } else {
            let postgresql_json_type_record_vec = serde_json::from_str::<std::vec::Vec<PostgresqlJsonTypeRecord>>(&input_token_stream.to_string()).expect("failed to get Config for generate_postgresql_json_types");
            {
                let mut acc = vec![];
                for element in &postgresql_json_type_record_vec {
                    if acc.contains(&element) {
                        panic!("not unique postgersql type provided: {element:#?}");
                    } else {
                        acc.push(element);
                    }
                }
            }
            let expanded_postgresql_json_type_record_vec = postgresql_json_type_record_vec.into_iter().fold(vec![], |mut acc, postgresql_json_type_record_element| {
                use postgresql_crud_macros_common::NotNullOrNullable;
                #[derive(Clone)]
                struct PostgresqlJsonTypeRecordHandle {
                    not_null_or_nullable: NotNullOrNullable,
                    postgresql_json_type_pattern: PostgresqlJsonTypePattern,
                }
                fn generate_postgresql_json_type_record_handle_vec(postgresql_json_type_record_handle: PostgresqlJsonTypeRecordHandle) -> std::vec::Vec<PostgresqlJsonTypeRecordHandle> {
                    let generate_vec = |current_postgresql_json_type_record_handle: PostgresqlJsonTypeRecordHandle|{
                        let mut acc = vec![];
                        for element in generate_postgresql_json_type_record_handle_vec(current_postgresql_json_type_record_handle) {
                            acc.push(element);
                        }
                        acc.push(postgresql_json_type_record_handle.clone());
                        acc
                    };
                    match (&postgresql_json_type_record_handle.not_null_or_nullable, &postgresql_json_type_record_handle.postgresql_json_type_pattern) {
                        (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) => vec![postgresql_json_type_record_handle],
                        (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::Standart) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::Standart,
                        }),
                        (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: dimension1_not_null_or_nullable.clone(),
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::Standart,
                        }),
                        (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension1 { .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.clone(),
                        }),
                        (
                            NotNullOrNullable::NotNull,
                            PostgresqlJsonTypePattern::ArrayDimension2 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                            },
                        ) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: dimension1_not_null_or_nullable.clone(),
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension1 {
                                dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                            },
                        }),
                        (
                            NotNullOrNullable::Nullable,
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. },
                        ) => generate_vec(PostgresqlJsonTypeRecordHandle {
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
                            not_null_or_nullable: dimension1_not_null_or_nullable.clone(),
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension2 {
                                dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                                dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                            },
                        }),
                        (
                            NotNullOrNullable::Nullable,
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. },
                        ) => generate_vec(PostgresqlJsonTypeRecordHandle {
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
                            not_null_or_nullable: dimension1_not_null_or_nullable.clone(),
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                                dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                                dimension3_not_null_or_nullable: dimension4_not_null_or_nullable.clone(),
                            },
                        }),
                        (
                            NotNullOrNullable::Nullable,
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. },
                        ) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.clone(),
                        }),
                    }
                }
                generate_postgresql_json_type_record_handle_vec(PostgresqlJsonTypeRecordHandle {
                    not_null_or_nullable: postgresql_json_type_record_element.not_null_or_nullable,
                    postgresql_json_type_pattern: postgresql_json_type_record_element.postgresql_json_type_pattern,
                }).into_iter().for_each(|postgresql_json_type_record_handle_element|{
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
            });
            // {
            //     let mut debug_vec = vec![];
            //     for element in &expanded_postgresql_json_type_record_vec {
            //         if let PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber = &element.postgresql_json_type {
            //             debug_vec.push(element);
            //         }
            //     }
            //     println!("{debug_vec:#?}");
            //     println!("{}", debug_vec.len());
            // }
            expanded_postgresql_json_type_record_vec
        }
    }
    // .into_iter()
    // .filter(|element| {
    //     use postgresql_crud_macros_common::NotNullOrNullable;
    //     let postgresql_type_filter = match &element.postgresql_json_type {
    //         PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => true,
    //         PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => true,
    //         PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => true,
    //         PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => true,
    //         PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => true,
    //         PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => true,
    //         PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => true,
    //         PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => true,
    //         PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => true,
    //         PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => true,
    //         PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => true,
    //         PostgresqlJsonType::StdStringStringAsJsonbString => true,
    //         PostgresqlJsonType::UuidUuidAsJsonbString => true,
    //     };
    //     let not_null_or_nullable_filter = match &element.not_null_or_nullable {
    //         NotNullOrNullable::NotNull => true,
    //         NotNullOrNullable::Nullable => true,
    //     };
    //     let postgresql_json_type_pattern_filter = match &element.postgresql_json_type_pattern {
    //         PostgresqlJsonTypePattern::Standart => true,
    //         PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &dimension1_not_null_or_nullable {
    //             NotNullOrNullable::NotNull => true,
    //             NotNullOrNullable::Nullable => true,
    //         },
    //         PostgresqlJsonTypePattern::ArrayDimension2 {
    //             dimension1_not_null_or_nullable,
    //             dimension2_not_null_or_nullable,
    //         } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable) {
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => true,
    //         },
    //         PostgresqlJsonTypePattern::ArrayDimension3 {
    //             dimension1_not_null_or_nullable,
    //             dimension2_not_null_or_nullable,
    //             dimension3_not_null_or_nullable,
    //         } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable) {
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => true,
    //         },
    //         PostgresqlJsonTypePattern::ArrayDimension4 {
    //             dimension1_not_null_or_nullable,
    //             dimension2_not_null_or_nullable,
    //             dimension3_not_null_or_nullable,
    //             dimension4_not_null_or_nullable,
    //         } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
    //             (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => true,
    //         },
    //     };
    //     postgresql_type_filter && not_null_or_nullable_filter && postgresql_json_type_pattern_filter
    // })
    // .collect::<std::vec::Vec<PostgresqlJsonTypeRecord>>()
    ;
    // macros_helpers::write_string_into_file::write_string_into_file(
    //     "GeneratePostgresqlJsonTypesJsonVariants",
    //     &serde_json::to_string(&postgresql_json_type_record_vec).unwrap(),
    // );
    use rayon::iter::IntoParallelRefIterator;
    use rayon::iter::ParallelIterator;
    let (postgresql_crud_json_object_rust_struct_fields_token_stream, postgresql_json_type_array) = postgresql_json_type_record_vec
        .par_iter()
        // .into_iter() //just for console prints ordering
        .map(|element| {
            // println!("{element:#?}");
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

            let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
            let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;

            let generate_ident_token_stream = |not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
                let vec_of_upper_camel_case = naming::VecOfUpperCamelCase;
                let array_of_upper_camel_case = naming::ArrayOfUpperCamelCase;
                let not_null_or_nullable_rust = not_null_or_nullable.rust();
                let (rust_part, postgresql_part) = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => (format!("{rust_type_name}"), format!("{postgresql_json_type_name}")),
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                        let d1 = dimension1_not_null_or_nullable;
                        let d1_rust = dimension1_not_null_or_nullable.rust();
                        (format!("{vec_of_upper_camel_case}{d1_rust}{rust_type_name}"), format!("{array_of_upper_camel_case}{d1}{postgresql_json_type_name}"))
                    }
                    PostgresqlJsonTypePattern::ArrayDimension2 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                    } => {
                        let d1 = dimension1_not_null_or_nullable;
                        let d1_rust = dimension1_not_null_or_nullable.rust();
                        let d2 = dimension2_not_null_or_nullable;
                        let d2_rust = dimension2_not_null_or_nullable.rust();
                        (
                            format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{rust_type_name}"),
                            format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{postgresql_json_type_name}"),
                        )
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
            let field_type_standart_not_null = match &postgresql_json_type {
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
            let generate_current_ident_origin_non_wrapping = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
                naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(current_not_null_or_nullable, current_postgresql_json_type_pattern))
            };
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
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => &field_type_standart_not_null,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                        let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &PostgresqlJsonTypePattern::Standart),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        };
                        generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    },
                    PostgresqlJsonTypePattern::ArrayDimension2 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                    } => &{
                        let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                                dimension1_not_null_or_nullable,
                                &PostgresqlJsonTypePattern::ArrayDimension1 {
                                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                },
                            ),
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
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &field_type_standart_not_null,
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &quote::quote! {std::option::Option<#field_type_standart_not_null>},
                },
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote! {#field_type_standart_not_null});
                    not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension1_type>})
                },
                PostgresqlJsonTypePattern::ArrayDimension2 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                } => &{
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(quote::quote! {#field_type_standart_not_null});
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension2_type>});
                    not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension1_type>})
                },
                PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                } => &{
                    let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(quote::quote! {#field_type_standart_not_null});
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension3_type>});
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension2_type>});
                    not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension1_type>})
                },
                PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                } => &{
                    let dimension4_type = dimension4_not_null_or_nullable.maybe_option_wrap(quote::quote! {#field_type_standart_not_null});
                    let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension4_type>});
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension3_type>});
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension2_type>});
                    not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension1_type>})
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
                    maximum: Some(#field_type_standart_not_null ::MAX as std::primitive::f64),
                    exclusive_maximum: None,
                    minimum: Some(#field_type_standart_not_null ::MIN as std::primitive::f64),
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
                    // let maybe_derive_partial_ord_token_stream = {
                    //     let partial_ord_comma_token_stream = quote::quote! {};
                    //     if let (
                    //         postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    //         PostgresqlJsonTypePattern::Standart
                    //     ) = (
                    //         &not_null_or_nullable,
                    //         &postgresql_json_type_pattern
                    //     ) {
                    //         match &postgresql_json_type {
                    //             PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::StdStringStringAsJsonbString => partial_ord_comma_token_stream,
                    //             PostgresqlJsonType::UuidUuidAsJsonbString => partial_ord_comma_token_stream,
                    //         }
                    //     }
                    //     else {
                    //         proc_macro2::TokenStream::new()
                    //     }
                    // };
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
                                PostgresqlJsonTypePattern::ArrayDimension2 {
                                    dimension1_not_null_or_nullable,
                                    dimension2_not_null_or_nullable,
                                } => generate_array_dimensions_initialization_token_stream(&{
                                    let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                                            &PostgresqlJsonTypePattern::ArrayDimension1 {
                                                dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                            },
                                            dimension1_not_null_or_nullable,
                                        ),
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
                    let maybe_additional_uuid_standart_not_null_impl_token_stream =
                        if let (PostgresqlJsonType::UuidUuidAsJsonbString, postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) = (&postgresql_json_type, &not_null_or_nullable, &postgresql_json_type_pattern) {
                            //need for uuid bind for json object logic//need for uuid bind for json object logic
                            let query_bind_as_postgresql_text_token_stream = quote::quote! {
                                pub fn query_bind_as_postgresql_text(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                                    query.bind(self.0.to_string())
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
                        let schema_id_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("postgersql_crud::postgersql_json_type::{ident_origin_upper_camel_case}"));
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
                let impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream =
                    macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self:#?}")});
                let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream =
                    postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_origin_upper_camel_case, &{
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
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                            },
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
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
                                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self.0.clone()), buf)//todo maybe remove .clone()?
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
                    #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream
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
                                    #dimension_number_pagination_token_stream: crate::Pagination
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
                let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream = postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_select_upper_camel_case,
                    &match ArrayDimension::try_from(postgresql_json_type_pattern) {
                        Ok(array_dimension) => {
                            let mut arguments_token_stream = vec![];
                            for element in 1..=array_dimension.to_usize() {
                                let dimension_number_pagination_token_stream = format!("dimension{element}_pagination").parse::<proc_macro2::TokenStream>().unwrap();
                                arguments_token_stream.push(quote::quote! {
                                    #dimension_number_pagination_token_stream: #crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
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
                    #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream
                }
            };
            let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
            let ident_where_element_token_stream = match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream_second(
                    &{
                        // let generate_where_element_variants_types_generic_token_stream = |
                        //     is_relevant_only_for_not_null: std::primitive::bool
                        // | -> &dyn naming::StdFmtDisplayPlusQuoteToTokens {
                        //     if is_relevant_only_for_not_null {
                        //         &ident_standart_not_null_origin_upper_camel_case
                        //     } else {
                        //         &ident_origin_upper_camel_case
                        //     }
                        // };
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
                        let common_postgresql_json_type_filters = vec![
                            postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal { ident: quote::quote! {#ident_origin_upper_camel_case} }
                        ];
                        let ident_origin_upper_camel_case_token_stream = quote::quote! {#ident_origin_upper_camel_case};
                        // ContainsAllElementsOfArray {
                        //     ident: proc_macro2::TokenStream,
                        // },
                        // // ContainedInArray,
                        // OverlapsWithArray {
                        //     ident: proc_macro2::TokenStream,
                        // },
                        // AllElementsEqual {
                        //     ident: proc_macro2::TokenStream,
                        // },
                        // ContainsElementGreaterThan {
                        //     ident: proc_macro2::TokenStream,
                        // },
                        // AllElementsGreaterThan {
                        //     ident: proc_macro2::TokenStream,
                        // },
                        match &element.postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan {
                                        ident: ident_origin_upper_camel_case_token_stream.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between {
                                        ident: ident_origin_upper_camel_case_token_stream.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In {
                                        ident: ident_origin_upper_camel_case_token_stream.clone(),
                                    });
                                    vec
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::RegularExpression);
                                    vec
                                }
                            },
                            //todo reuse analog filters in generate_postgresql_types
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                                let array_dimension1_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(
                                        &generate_ident_token_stream(&dimension1_not_null_or_nullable, &PostgresqlJsonTypePattern::Standart)
                                    );
                                    quote::quote!{#value}
                                };
                                let common_array_dimension1_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneOverlapsWithArray {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension1_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionGreaterThan {
                                            ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters
                                    },
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension1_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension1_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementRegularExpression);
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsRegularExpression);
                                        filters
                                    }
                                }
                            }
                            PostgresqlJsonTypePattern::ArrayDimension2 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                            } => {
                                let array_dimension1_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        &dimension1_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::ArrayDimension1 {
                                            dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone()
                                        }
                                    ));
                                    quote::quote!{#value}
                                };
                                let array_dimension2_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        &dimension2_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::Standart
                                    ));
                                    quote::quote!{#value}
                                };
                                let common_array_dimension2_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionEqual {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension2_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionGreaterThan {
                                            ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters
                                    },
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension2_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension2_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionRegularExpression);
                                        filters
                                    },
                                }
                            },
                            PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                            } => {
                                let array_dimension1_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        &dimension1_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::ArrayDimension2 {
                                            dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                                            dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone()
                                        }
                                    ));
                                    quote::quote!{#value}
                                };
                                let array_dimension2_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        &dimension2_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::ArrayDimension1 {
                                            dimension1_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                                        }
                                    ));
                                    quote::quote!{#value}
                                };
                                let array_dimension3_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        &dimension3_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::Standart
                                    ));
                                    quote::quote!{#value}
                                };
                                let common_array_dimension3_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionEqual {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreePositionEqual {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthMoreThan);
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension3_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreePositionGreaterThan {
                                            ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters
                                    },
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension3_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension3_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreePositionRegularExpression);
                                        filters
                                    },
                                }
                            },
                            PostgresqlJsonTypePattern::ArrayDimension4 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                                dimension4_not_null_or_nullable
                            } => {
                                let array_dimension1_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        &dimension1_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::ArrayDimension3 {
                                            dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                                            dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                                            dimension3_not_null_or_nullable: dimension4_not_null_or_nullable.clone()
                                        }
                                    ));
                                    quote::quote!{#value}
                                };
                                let array_dimension2_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        &dimension2_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::ArrayDimension2 {
                                            dimension1_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                                            dimension2_not_null_or_nullable: dimension4_not_null_or_nullable.clone(),
                                        }
                                    ));
                                    quote::quote!{#value}
                                };
                                let array_dimension3_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        &dimension3_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::ArrayDimension1 {
                                            dimension1_not_null_or_nullable: dimension4_not_null_or_nullable.clone(),
                                        }
                                    ));
                                    quote::quote!{#value}
                                };
                                let array_dimension4_inner_element_ident_origin_upper_camel_case = {
                                    let value = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                                        &dimension4_not_null_or_nullable,
                                        &PostgresqlJsonTypePattern::Standart
                                    ));
                                    quote::quote!{#value}
                                };
                                let common_array_dimension4_postgresql_json_type_filters = {
                                    let mut vec = common_postgresql_json_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOnePositionEqual {
                                        ident: array_dimension1_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoPositionEqual {
                                        ident: array_dimension2_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreePositionEqual {
                                        ident: array_dimension3_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourPositionEqual {
                                        ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone()
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
                                        ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone()
                                    });
                                    vec
                                };
                                match &postgresql_json_type_specific {
                                    PostgresqlJsonTypeSpecific::Number => {
                                        let mut filters = common_array_dimension4_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourPositionGreaterThan {
                                            ident: array_dimension4_inner_element_ident_origin_upper_camel_case.clone(),
                                        });
                                        filters
                                    },
                                    PostgresqlJsonTypeSpecific::Bool => common_array_dimension4_postgresql_json_type_filters,
                                    PostgresqlJsonTypeSpecific::String => {
                                        let mut filters = common_array_dimension4_postgresql_json_type_filters.clone();
                                        filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourPositionRegularExpression);
                                        filters
                                    },
                                }
                            },
                        }
                    },
                    &ident,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                    &postgresql_crud_macros_common::IsQueryBindMutable::False
                ),
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                    let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
                    let ident_as_token_stream = generate_ident_token_stream(&postgresql_crud_macros_common::NotNullOrNullable::NotNull, &element.postgresql_json_type_pattern);
                    quote::quote! {
                        pub type #ident_where_element_upper_camel_case = crate::NullableJsonObjectPostgresqlTypeWhereFilter<
                            <#ident_as_token_stream as crate::PostgresqlJsonType>::WhereElement
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
                let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_read_upper_camel_case,
                    &quote::quote! {Self(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                );
                quote::quote! {
                    #ident_read_token_stream
                    #impl_ident_read_token_stream
                    #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream
                }
            };
            // println!("{ident_read_token_stream}");
            let ident_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident);
            let ident_read_inner_token_stream = quote::quote! {
                pub type #ident_read_inner_upper_camel_case = #ident_origin_impl_new_value_type_token_stream;
            };
            let impl_crate_postgresql_json_type_for_ident_token_stream = {
                let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
                let generate_dimension_number_stringified = |dimensions_number: std::primitive::usize| format!("dimension{dimensions_number}");
                let generate_dimension_number_start_stringified = |dimensions_number: std::primitive::usize| format!("{}_start", generate_dimension_number_stringified(dimensions_number));
                let generate_dimension_number_end_stringified = |dimensions_number: std::primitive::usize| format!("{}_end", generate_dimension_number_stringified(dimensions_number));
                postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
                    &postgresql_crud_macros_common::ImportPath::Crate,
                    &ident,
                    &ident_origin_upper_camel_case,
                    &ident_origin_upper_camel_case,
                    &postgresql_crud_macros_common::IsCreateQueryPartSelfCreateUsed::False,
                    &quote::quote! {
                        match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                Ok(format!("${increment}"))
                            }
                            None => Err(crate::QueryPartErrorNamed::#checked_add_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!()
                            }),
                        }
                    },
                    &postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                    &quote::quote! {
                        query = query.bind(#value_snake_case);
                        query
                    },
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
                                NotNullOrNullable::Nullable => format!("case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident})='null' then null else ({format_handle}) end"),
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
                                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(&dimension1_not_null_or_nullable),
                                PostgresqlJsonTypePattern::ArrayDimension2 {
                                    dimension1_not_null_or_nullable,
                                    dimension2_not_null_or_nullable,
                                } => {
                                    let dimension2_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(&dimension2_not_null_or_nullable);
                                    generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable_content_token_stream)
                                }
                                PostgresqlJsonTypePattern::ArrayDimension3 {
                                    dimension1_not_null_or_nullable,
                                    dimension2_not_null_or_nullable,
                                    dimension3_not_null_or_nullable,
                                } => {
                                    let dimension3_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(&dimension3_not_null_or_nullable);
                                    let dimension2_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(&dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable_content_token_stream);
                                    generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable_content_token_stream)
                                }
                                PostgresqlJsonTypePattern::ArrayDimension4 {
                                    dimension1_not_null_or_nullable,
                                    dimension2_not_null_or_nullable,
                                    dimension3_not_null_or_nullable,
                                    dimension4_not_null_or_nullable,
                                } => {
                                    let dimension4_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(&dimension4_not_null_or_nullable);
                                    let dimension3_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(&dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable_content_token_stream);
                                    let dimension2_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(&dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable_content_token_stream);
                                    generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable_content_token_stream)
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
                    &postgresql_crud_macros_common::IsUpdateQueryPartSelfUpdateUsed::False,
                    &postgresql_crud_macros_common::IsUpdateQueryPartJsonbSetTargetUsed::False,
                    &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                    &quote::quote! {
                        query = query.bind(#value_snake_case);
                        query
                    },
                )
            };
            let generated = quote::quote! {
                #ident_token_stream
                #ident_origin_token_stream
                #ident_select_token_stream
                #ident_where_element_token_stream
                #ident_read_token_stream
                #ident_read_inner_token_stream
                #impl_crate_postgresql_json_type_for_ident_token_stream
            };
            // if let (
            //     PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber,
            //     // PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber,
            //     // PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber,
            //     // PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber,
            //     // PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber,
            //     // PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber,
            //     // PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber,
            //     // PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber,
            //     // PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber,
            //     // PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber,
            //     // PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean,
            //     // PostgresqlJsonType::StdStringStringAsJsonbString,
            //     // PostgresqlJsonType::UuidUuidAsJsonbString,

            //     // postgresql_crud_macros_common::NotNullOrNullable::NotNull,
            //     postgresql_crud_macros_common::NotNullOrNullable::Nullable,

            //     PostgresqlJsonTypePattern::Standart,
            //     // PostgresqlJsonTypePattern::ArrayDimension1 {
            //     //     dimension1_not_null_or_nullable,
            //     // },
            //     // PostgresqlJsonTypePattern::ArrayDimension2 {
            //     //     dimension1_not_null_or_nullable,
            //     //     dimension2_not_null_or_nullable,
            //     // },
            //     // PostgresqlJsonTypePattern::ArrayDimension3 {
            //     //     dimension1_not_null_or_nullable,
            //     //     dimension2_not_null_or_nullable,
            //     //     dimension3_not_null_or_nullable,
            //     // },
            //     // PostgresqlJsonTypePattern::ArrayDimension4 {
            //     //     dimension1_not_null_or_nullable,
            //     //     dimension2_not_null_or_nullable,
            //     //     dimension3_not_null_or_nullable,
            //     //     dimension4_not_null_or_nullable,
            //     // }
            // ) = (
            //     &postgresql_json_type,
            //     &not_null_or_nullable,
            //     &postgresql_json_type_pattern
            // ) {
            //     // use postgresql_crud_macros_common::NotNullOrNullable;
            //     // let d1 = match &dimension1_not_null_or_nullable {
            //     //     NotNullOrNullable::NotNull => true,
            //     //     NotNullOrNullable::Nullable => false,
            //     // };
            //     // let d2 = match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable) {
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     // };
            //     // let d3 = match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable) {
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => true,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     // };
            //     // let d4 = match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     // };
            //     // if d3 {
            //         macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            //             "PostgresqlJsonTypeTokens",
            //             &generated,
            //         );
            //     // }
            // }
            (
                {
                    let field_ident = format!("column_{}", uuid::Uuid::new_v4()).replace("-", "_").parse::<proc_macro2::TokenStream>().unwrap();
                    quote::quote! {
                        pub #field_ident: postgresql_crud::postgresql_json_type:: #ident,
                    }
                    .to_string()
                },
                generated.to_string(),
            )
        })
        .collect::<(std::vec::Vec<String>, std::vec::Vec<String>)>();
    if false {
        let postgresql_crud_json_object_rust_struct_fields_token_stream = postgresql_crud_json_object_rust_struct_fields_token_stream
            .into_iter()
            .map(|element| element.parse::<proc_macro2::TokenStream>().unwrap())
            .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            "PostgresqlJsonTypeTokensExampleStruct",
            &quote::quote! {
                struct Example {
                    #(#postgresql_crud_json_object_rust_struct_fields_token_stream)*
                }
            },
        );
    }
    let generated = {
        let postgresql_json_type_array = postgresql_json_type_array.into_iter().map(|element| element.parse::<proc_macro2::TokenStream>().unwrap()).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#postgresql_json_type_array)*}
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlJsonTypeTokens",
    //     &generated
    // );
    generated.into()
}
