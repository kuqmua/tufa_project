// generate_postgresql_types::generate_postgresql_types!("All");
generate_postgresql_types::generate_postgresql_types!({
    "Concrete": [
        {
            "postgresql_type": "StdPrimitiveI16AsInt2",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        },
        {
            "postgresql_type": "StdPrimitiveI16AsInt2",
            "not_null_or_nullable": "Nullable",
            "postgresql_type_pattern": "Standart"
        },
        {
            "postgresql_type": "StdPrimitiveI16AsInt2",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": {
                "ArrayDimension1": {
                    "dimension1_not_null_or_nullable": "NotNull"
                }
            }
        },
        {
            "postgresql_type": "StdPrimitiveI16AsInt2",
            "not_null_or_nullable": "Nullable",
            "postgresql_type_pattern": {
                "ArrayDimension1": {
                    "dimension1_not_null_or_nullable": "NotNull"
                }
            }
        },
        {
            "postgresql_type": "StdPrimitiveI16AsInt2",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": {
                "ArrayDimension1": {
                    "dimension1_not_null_or_nullable": "Nullable"
                }
            }
        },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI32AsInt4",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI32AsInt4",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI32AsInt4",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI32AsInt4",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI32AsInt4",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI32AsInt4",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI64AsInt8",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI64AsInt8",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI64AsInt8",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI64AsInt8",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI64AsInt8",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI64AsInt8",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF32AsFloat4",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF32AsFloat4",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF32AsFloat4",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF32AsFloat4",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF32AsFloat4",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF32AsFloat4",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF64AsFloat8",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF64AsFloat8",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF64AsFloat8",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF64AsFloat8",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF64AsFloat8",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveF64AsFloat8",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsSmallSerialInitializedByPostgresql",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI32AsSerialInitializedByPostgresql",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI64AsBigSerialInitializedByPostgresql",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveBoolAsBool",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveBoolAsBool",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveBoolAsBool",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveBoolAsBool",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveBoolAsBool",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveBoolAsBool",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdStringStringAsText",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdStringStringAsText",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdStringStringAsText",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdStringStringAsText",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdStringStringAsText",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdStringStringAsText",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        {
            "postgresql_type": "SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        }
        // ,
        // {
        //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // }
    ]
});

fn maybe_primary_key(is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
    if is_primary_key { "primary key" } else { "" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct PaginationStartsWithOne(postgresql_crud_common::PaginationBase);
#[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum PaginationStartsWithOneTryNewErrorNamed {
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::i64,
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    LimitIsLessThanOrEqualToZero {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OffsetIsLessThanOne {
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PaginationStartsWithOne {
    pub fn try_new(limit: std::primitive::i64, offset: std::primitive::i64) -> Result<Self, PaginationStartsWithOneTryNewErrorNamed> {
        if limit <= 0 || offset < 1 {
            if limit <= 0 {
                Err(PaginationStartsWithOneTryNewErrorNamed::LimitIsLessThanOrEqualToZero { limit, code_occurence: error_occurence_lib::code_occurence!() })
            } else {
                Err(PaginationStartsWithOneTryNewErrorNamed::OffsetIsLessThanOne { offset, code_occurence: error_occurence_lib::code_occurence!() })
            }
        } else if offset.checked_add(limit).is_some() {
            Ok(Self(postgresql_crud_common::PaginationBase::new_unchecked(limit, offset)))
        } else {
            Err(PaginationStartsWithOneTryNewErrorNamed::OffsetPlusLimitIsIntOverflow { limit, offset, code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
    pub const fn start(&self) -> std::primitive::i64 {
        self.0.start()
    }
    pub const fn end(&self) -> std::primitive::i64 {
        self.0.end()
    }
}
impl<'de> serde::Deserialize<'de> for PaginationStartsWithOne {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[expect(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__f, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "limit" => serde::__private::Ok(__Field::__field0),
                    "offset" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"limit" => serde::__private::Ok(__Field::__field0),
                    b"offset" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PaginationStartsWithOne>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PaginationStartsWithOne;
            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__f, "struct PaginationStartsWithOne")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PaginationStartsWithOne with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct PaginationStartsWithOne with 2 elements"));
                    }
                };
                match PaginationStartsWithOne::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::primitive::i64> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::primitive::i64> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("limit"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("offset"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("limit")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("offset")?,
                };
                match PaginationStartsWithOne::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &[&str] = &["limit", "offset"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "PaginationStartsWithOne",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<Self>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PaginationStartsWithOne {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        self.0.query_part(increment, column, is_need_to_add_logical_operator)
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
        sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
        std::string::String
    > {
        self.0.query_bind(query)
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PaginationStartsWithOne {
    #[inline]
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::PaginationBase::new_unchecked(
            postgresql_crud_common::DEFAULT_PAGINATION_LIMIT,
            1
        ))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for PaginationStartsWithOne {
    #[inline]
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        let one = 1;
        Self(postgresql_crud_common::PaginationBase::new_unchecked(
            std::i32::MAX.checked_sub(one).expect("error c0f03c51-d565-4377-ad4e-f38ee636909b").into(),
            one.into()
        ))
    }
}
/////////////
// #[derive(Debug)]
// pub struct OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2;
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// struct OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin(std::option::Option<VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin>);
// impl OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin {
//     pub fn new(value: std::option::Option<std::vec::Vec<std::primitive::i16>>) -> Self {
//         Self(match value {
//             Some(value) => Some(VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin::new(value)),
//             None => None,
//         })
//     }
// }
// impl std::convert::Into<std::option::Option<std::vec::Vec<std::primitive::i16>>> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin {
//     fn into(self) -> std::option::Option<std::vec::Vec<std::primitive::i16>> {
//         match self.0 {
//             Some(value) => Some(value.0.into_iter().map(|element| element.0).collect()),
//             None => None,
//         }
//     }
// }
// impl std::fmt::Display for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{self:?}")
//     }
// }
// impl error_occurence_lib::ToStdStringString for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin {
//     fn to_std_string_string(&self) -> std::string::String {
//         self.to_string()
//     }
// }
// impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(Some(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <std::option::Option<VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin> as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <std::option::Option<VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <std::option::Option<VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
// impl sqlx::postgres::PgHasArrayType for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin {
//     fn array_type_info() -> sqlx::postgres::PgTypeInfo {
//         <std::primitive::i16 as sqlx::postgres::PgHasArrayType>::array_type_info()
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin);
// impl OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration {
//     pub fn new(value: std::option::Option<std::vec::Vec<std::primitive::i16>>) -> Self {
//         Self(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin::new(value))
//     }
// }
// impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Create(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin);
// impl OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Create {
//     pub fn new(value: std::option::Option<std::vec::Vec<std::primitive::i16>>) -> Self {
//         Self(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin::new(value))
//     }
// }
// impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Create {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
//     }
// }
// impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Create {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Create {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// #[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Select {
//     dimension1_pagination: crate::PaginationStartsWithOne,
// }
// impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Select {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self {
//             dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//         }
//     }
// }
// impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Select {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
//         Self {
//             dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub enum OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2WhereElement {
//     Equal(where_element_filters::PostgresqlTypeWhereElementEqual<OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration>),
//     DimensionOneEqual(where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<StdPrimitiveI16AsNotNullInt2TableTypeDeclaration>),
//     DimensionOneLengthEqual(where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
//     DimensionOneLengthMoreThan(where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
//     DimensionOneGreaterThan(where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThan<StdPrimitiveI16AsNotNullInt2TableTypeDeclaration>),
//     DimensionOneBetween(where_element_filters::PostgresqlTypeWhereElementDimensionOneBetween<StdPrimitiveI16AsNotNullInt2TableTypeDeclaration>),
//     DimensionOneIn(where_element_filters::PostgresqlTypeWhereElementDimensionOneIn<OptionStdPrimitiveI16AsNullableInt2TableTypeDeclaration>),
// }
// impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2WhereElement {
//     fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
//         match &self {
//             Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::DimensionOneGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::DimensionOneBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//         }
//     }
//     fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
//         match self {
//             Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::DimensionOneGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::DimensionOneBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
//         }
//     }
// }
// impl error_occurence_lib::ToStdStringString for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2WhereElement {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl postgresql_crud_common::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2WhereElement {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         vec![
//             Self::Equal(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::DimensionOneEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::DimensionOneLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::DimensionOneLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::DimensionOneGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::DimensionOneBetween(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::DimensionOneIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//         ]
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin);
// impl OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read {
//     pub fn new(value: std::option::Option<std::vec::Vec<std::primitive::i16>>) -> Self {
//         Self(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin::new(value))
//     }
// }
// impl error_occurence_lib::ToStdStringString for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read {
//     fn to_std_string_string(&self) -> std::string::String {
//         self.0.to_string()
//     }
// }
// impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
//     }
// }
// impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// pub type OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2ReadInner = std::option::Option<std::vec::Vec<std::primitive::i16>>;
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Update(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin);
// impl OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Update {
//     pub fn new(value: std::option::Option<std::vec::Vec<std::primitive::i16>>) -> Self {
//         Self(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin::new(value))
//     }
// }
// impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Update {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
//     }
// }
// impl error_occurence_lib::ToStdStringString for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Update {
//     fn to_std_string_string(&self) -> std::string::String {
//         self.0.to_std_string_string()
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2UpdateForQuery(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin);
// impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2UpdateForQuery {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2UpdateForQuery {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
// }
// impl std::convert::From<OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Update> for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2UpdateForQuery {
//     fn from(value: OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Update) -> Self {
//         Self(value.0)
//     }
// }
// impl postgresql_crud_common::PostgresqlType for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2 {
//     type TableTypeDeclaration = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration;
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} int2[],check (array_position({column},null) is null)")
//     }
//     type Create = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Create;
//     fn create_query_part(_: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
//         let mut acc = std::string::String::default();
//         match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
//             Ok(value) => {
//                 acc.push_str(&format!("${value}"));
//             }
//             Err(error) => {
//                 return Err(error);
//             }
//         }
//         Ok(acc)
//     }
//     fn create_query_bind(value: Self::Create, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
//         let value = match value.0 .0 {
//             Some(value) => Some(value),
//             None => None,
//         };
//         if let Err(error) = query.try_bind(value) {
//             return Err(error.to_string());
//         }
//         Ok(query)
//     }
//     type Select = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Select;
//     fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
//         format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
//     }
//     type WhereElement = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2WhereElement;
//     type Read = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read;
//     fn normalize(value: Self::Read) -> Self::Read {
//         OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Origin(match value.0 .0 {
//             Some(value) => Some(<VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud_common::PostgresqlType>::normalize(VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Read(value)).0),
//             None => None,
//         }))
//     }
//     type ReadOnlyIds = postgresql_crud_common::NonPrimaryKeyPostgresqlTypeReadOnlyIds;
//     fn select_only_ids_query_part(column: &std::primitive::str) -> std::string::String {
//         format!("'{{\"value\": null}}'::jsonb as {column},")
//     }
//     type ReadInner = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2ReadInner;
//     fn into_inner(value: Self::Read) -> Self::ReadInner {
//         match value.0 .0 {
//             Some(value) => Some(value.0.into_iter().map(|element| element.0).collect()),
//             None => None,
//         }
//     }
//     type Update = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Update;
//     type UpdateForQuery = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2UpdateForQuery;
//     fn update_query_part(_: &Self::UpdateForQuery, _: &std::primitive::str, _: &std::primitive::str, _: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
//         let mut acc = std::string::String::default();
//         match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
//             Ok(value) => {
//                 acc.push_str(&format!("${value}"));
//             }
//             Err(error) => {
//                 return Err(error);
//             }
//         }
//         Ok(acc)
//     }
//     fn update_query_bind<'a>(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
//         let value = match value.0 .0 {
//             Some(value) => Some(value),
//             None => None,
//         };
//         if let Err(error) = query.try_bind(value) {
//             return Err(error.to_string());
//         }
//         Ok(query)
//     }
//     fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, column: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
//         Ok(format!("'{{\"value\": null}}'::jsonb as {column},"))
//     }
//     fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
//         Ok(query)
//     }
// }
// #[cfg(feature = "test-utils")]
// impl postgresql_crud_common::PostgresqlTypeTestCases for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2 {
//     type PostgresqlType = Self;
//     type Select = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Select;
//     fn vec_create() -> std::vec::Vec<<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Create> {
//         let mut acc = vec![];
//         for element in <VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud_common::PostgresqlTypeTestCases>::vec_create() {
//             acc.push(<OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2 as postgresql_crud_common::PostgresqlType>::Create::new(Some(element.0.into())));
//         }
//         acc.push(<OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2 as postgresql_crud_common::PostgresqlType>::Create::new(None));
//         acc
//     }
//     fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadInner>> {
//         let mut acc = vec![];
//         let read_only_ids_to_two_dimensional_vec_read_inner = <VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud_common::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&read_only_ids);
//         let option_additional = {
//             let mut option_additional = None;
//             for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
//                 if option_additional.is_some() {
//                     break;
//                 }
//                 for element1 in element0 {
//                     if option_additional.is_some() {
//                         break;
//                     }
//                     for element2 in element1 {
//                         if option_additional.is_none() {
//                             option_additional = Some((vec![Some(vec![element2.clone()])], vec![Some(vec![element2.clone(), element2.clone()])]));
//                         } else {
//                             break;
//                         }
//                     }
//                 }
//             }
//             option_additional
//         };
//         let has_len_more_than_one = {
//             let mut has_len_more_than_one = false;
//             for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
//                 for element1 in element0 {
//                     if element1.len() > 1 {
//                         has_len_more_than_one = true;
//                         break;
//                     }
//                 }
//             }
//             has_len_more_than_one
//         };
//         acc.push(vec![Some({
//             let mut acc = vec![];
//             for element0 in read_only_ids_to_two_dimensional_vec_read_inner.clone() {
//                 for element1 in element0 {
//                     for element2 in element1 {
//                         acc.push(element2);
//                     }
//                 }
//             }
//             acc
//         })]);
//         acc.push(vec![None]);
//         if let Some(value) = option_additional {
//             if has_len_more_than_one {
//                 acc.push(value.0);
//             }
//             if !has_len_more_than_one {
//                 acc.push(value.1);
//             }
//         }
//         acc
//     }
//     fn read_inner_into_read_with_new_or_try_new_unwraped(value: std::option::Option<std::vec::Vec<std::primitive::i16>>) -> <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Read {
//         <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Read::new(value)
//     }
//     fn read_inner_into_update_with_new_or_try_new_unwraped(value: std::option::Option<std::vec::Vec<std::primitive::i16>>) -> <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Update {
//         <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Update::new(value)
//     }
//     fn update_to_read_only_ids(value: &<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Update) -> <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadOnlyIds {
//         postgresql_crud_common::NonPrimaryKeyPostgresqlTypeReadOnlyIds(postgresql_crud_common::Value { value: None })
//     }
//     fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Read>> {
//         Some(postgresql_crud_common::Value {
//             value: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::normalize(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//         })
//     }
//     fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Read, option_update: std::option::Option<<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Update>) -> <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Read {
//         <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::normalize(match option_update {
//             Some(value) => OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read(value.0),
//             None => read,
//         })
//     }
//     fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadOnlyIds, create: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Create) -> <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Read {
//         <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::normalize(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Read(create.0))
//     }
//     fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadOnlyIds, create: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Read>> {
//         Some(postgresql_crud_common::Value {
//             value: <OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2 as postgresql_crud_common::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
//         })
//     }
//     fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadOnlyIds, create: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Create) -> <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::TableTypeDeclaration {
//         OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration(create.0)
//     }
//     fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadOnlyIds, create: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Create) -> <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::WhereElement {
//         OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2WhereElement::Equal(where_element_filters::PostgresqlTypeWhereElementEqual {
//             logical_operator: postgresql_crud_common::LogicalOperator::Or,
//             value: OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration(create.0),
//         })
//     }
//     fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadOnlyIds, create: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Create) -> std::vec::Vec<<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::WhereElement> {
//         vec![OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2WhereElement::Equal(where_element_filters::PostgresqlTypeWhereElementEqual {
//             logical_operator: postgresql_crud_common::LogicalOperator::Or,
//             value: OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration(create.0),
//         })]
//     }
//     fn read_only_ids_merged_with_create_into_option_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadOnlyIds, create: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Create) -> std::option::Option<std::vec::Vec<<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::WhereElement>> {
//         None
//     }
//     fn read_only_ids_merged_with_create_into_postgresql_type_option_vec_where_element_dimension_one_equal(
//         read_only_ids: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::ReadOnlyIds,
//         create: <Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::Create,
//     ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlType as postgresql_crud_common::PostgresqlType>::WhereElement>> {
//         match create.0.0 {
//             Some(value) => Some({
//                 let mut acc = vec![];
//                 for (i, element) in value.0.into_iter().enumerate() {
//                     let index = i.checked_add(1).expect("error d69a5b02-cef0-45b7-8ef9-9d70f931c30d");
//                     acc.push(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2WhereElement::DimensionOneEqual(where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual {
//                         logical_operator: postgresql_crud_common::LogicalOperator::Or,
//                         dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::NotZeroUnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 2f2777c5-eac2-472e-a49e-6dfe385b574c")).expect("error 44345e1a-f278-4423-96e2-227cc7b7de0f")]).expect("error 8a75c3ac-d09c-4092-9059-5d10291cff22"),
//                         value: StdPrimitiveI16AsNotNullInt2TableTypeDeclaration(element)
//                     }));
//                 }
//                 acc
//             }),
//             None => None
//         }
//     }
// }
// impl postgresql_crud_common::PostgresqlTypeNotPrimaryKey for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2 {
//     type PostgresqlType = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2;
//     type Create = OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2Create;
// }
// impl postgresql_crud_common::PostgresqlTypeEqualOperator for OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2TableTypeDeclaration {
//     fn operator(&self) -> postgresql_crud_common::EqualOperator {
//         if self.0 .0.is_some() {
//             postgresql_crud_common::EqualOperator::Equal
//         } else {
//             postgresql_crud_common::EqualOperator::IsNull
//         }
//     }
// }
