// generate_postgresql_types::generate_postgresql_types!("All");
generate_postgresql_types::generate_postgresql_types!({
    "Concrete": [
        {
            "postgresql_type": "StdPrimitiveI16AsInt2",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        {
            "postgresql_type": "StdPrimitiveI32AsInt4",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        },
        {
            "postgresql_type": "StdPrimitiveI64AsInt8",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
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
        ,
        // {
        //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
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
        // },
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
        // }
        // ,
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
        // }
        // ,
        {
            "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        }
        ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        {
            "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        }
        // ,
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
        // }
        // ,
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
        // }
        // ,
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

////////////////////

// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
//     let pool = sqlx::PgPool::connect("postgres://postgres:postgres@127.0.0.1:5432/dev?connect_timeout=10").await?;
//     // let rows: Vec<TestRange> = sqlx::query_as::<_, TestRange>("SELECT * FROM test_ranges").fetch_all(&pool).await?;
//     // for row in rows {
//     //     println!("Row: {:?}", row);
//     // }
//     // Ok(())

//     // Connect to PostgreSQL

//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MIN),
//     //     end: Bound::Included(std::primitive::i32::MIN)
//     // };
//     // [-2147483648,-2147483647)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MIN),
//     //     end: Bound::Included(std::primitive::i32::MIN)
//     // };
//     //empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MIN),
//     //     end: Bound::Excluded(std::primitive::i32::MIN)
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MIN),
//     //     end: Bound::Excluded(std::primitive::i32::MIN)
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MIN),
//     //     end: Bound::Unbounded
//     // };
//     // [-2147483648,)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MIN),
//     //     end: Bound::Unbounded
//     // };
//     // [-2147483647,)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Unbounded,
//     //     end: Bound::Included(std::primitive::i32::MIN),
//     // };
//     // (,-2147483647)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Unbounded,
//     //     end: Bound::Excluded(std::primitive::i32::MIN),
//     // };
//     // (,-2147483648)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(0),
//     //     end: Bound::Included(0),
//     // };
//     // [0,1)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(0),
//     //     end: Bound::Included(0),
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(0),
//     //     end: Bound::Excluded(0),
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(0),
//     //     end: Bound::Excluded(0),
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(i32::MAX),
//     //     end: Bound::Included(i32::MAX),
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22003", message: "integer out of range", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1492), routine: Some("int4range_canonical") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(i32::MAX),
//     //     end: Bound::Included(i32::MAX),
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(i32::MAX),
//     //     end: Bound::Excluded(i32::MAX),
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(i32::MAX),
//     //     end: Bound::Excluded(i32::MAX),
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(0),
//     //     end: Bound::Included(1),
//     // };
//     // [0,2)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(0),
//     //     end: Bound::Included(1),
//     // };
//     // [1,2)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(0),
//     //     end: Bound::Excluded(1),
//     // };
//     // [0,1)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(0),
//     //     end: Bound::Excluded(1),
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(-1),
//     //     end: Bound::Included(0),
//     // };
//     // [-1,1)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(-1),
//     //     end: Bound::Included(0),
//     // };
//     // [0,1)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(-1),
//     //     end: Bound::Excluded(0),
//     // };
//     // [-1,0)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(-1),
//     //     end: Bound::Excluded(0),
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(20),
//     //     end: Bound::Included(10),
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(20),
//     //     end: Bound::Included(10),
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(20),
//     //     end: Bound::Excluded(10),
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(20),
//     //     end: Bound::Excluded(10),
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(2),
//     //     end: Bound::Included(1),
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(2),
//     //     end: Bound::Included(1),
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(2),
//     //     end: Bound::Excluded(1),
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(2),
//     //     end: Bound::Excluded(1),
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MIN + 1),
//     //     end: Bound::Included(std::primitive::i32::MIN)
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MIN + 1),
//     //     end: Bound::Included(std::primitive::i32::MIN)
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MIN + 1),
//     //     end: Bound::Excluded(std::primitive::i32::MIN)
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MIN + 1),
//     //     end: Bound::Excluded(std::primitive::i32::MIN)
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MIN),
//     //     end: Bound::Included(std::primitive::i32::MIN + 1)
//     // };
//     // [-2147483648,-2147483646)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MIN),
//     //     end: Bound::Included(std::primitive::i32::MIN + 1)
//     // };
//     // [-2147483647,-2147483646)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MIN),
//     //     end: Bound::Excluded(std::primitive::i32::MIN + 1)
//     // };
//     // [-2147483648,-2147483647)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MIN),
//     //     end: Bound::Excluded(std::primitive::i32::MIN + 1)
//     // };
//     // empty
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MAX),
//     //     end: Bound::Included(std::primitive::i32::MAX - 1)
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MAX),
//     //     end: Bound::Included(std::primitive::i32::MAX - 1)
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MAX),
//     //     end: Bound::Excluded(std::primitive::i32::MAX - 1)
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MAX),
//     //     end: Bound::Excluded(std::primitive::i32::MAX - 1)
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22000", message: "range lower bound must be less than or equal to range upper bound", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1748), routine: Some("range_serialize") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MAX - 1),
//     //     end: Bound::Included(std::primitive::i32::MAX)
//     // };
//     //HERE
//     // Error: Database(PgDatabaseError { severity: Error, code: "22003", message: "integer out of range", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1492), routine: Some("int4range_canonical") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MAX - 1),
//     //     end: Bound::Included(std::primitive::i32::MAX)
//     // };
//     // Error: Database(PgDatabaseError { severity: Error, code: "22003", message: "integer out of range", detail: None, hint: None, position: None, where: Some("unnamed portal parameter $1"), schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("rangetypes.c"), line: Some(1492), routine: Some("int4range_canonical") })
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Included(std::primitive::i32::MAX - 1),
//     //     end: Bound::Excluded(std::primitive::i32::MAX)
//     // };
//     // [2147483646,2147483647)
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Excluded(std::primitive::i32::MAX - 1),
//     //     end: Bound::Excluded(std::primitive::i32::MAX)
//     // };
//     // empty
//     ////////////
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Unbounded,
//     //     end: Bound::Included(std::primitive::i32::MAX),
//     // };
//     // /error
//     // let range: PgRange<i32> = PgRange {
//     //     start: Bound::Unbounded,
//     //     end: Bound::Excluded(std::primitive::i32::MAX),
//     // };
//     //value
//     println!("{range:#?}");

//     // Insert into the database
//     sqlx::query("INSERT INTO test_ranges (range_column) VALUES ($1)").bind(range).execute(&pool).await?;

//     println!("Range inserted!");

//     Ok(())
// }