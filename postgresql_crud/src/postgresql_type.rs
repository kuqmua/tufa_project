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
        // {
        //     "postgresql_type": "StdPrimitiveI32AsInt4",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI64AsInt8",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        {
            "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        },
        {
            "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        },
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
        {
            "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        },
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
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // }
        // ,
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


// #[derive(Debug, Clone, PartialEq)]
// pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange<StdPrimitiveI32AsNotNullInt4Origin>);
// #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence, schemars::JsonSchema)]
// pub enum SqlxPostgresTypesPgRangeStdPrimitiveI32TryNewErrorNamed {
//     IncludedStartMoreThanIncludedEnd {
//         #[eo_to_std_string_string_serialize_deserialize]
//         start: std::primitive::i32,
//         #[eo_to_std_string_string_serialize_deserialize]
//         end: std::primitive::i32,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     IncludedStartMoreThanExcludedEnd {
//         #[eo_to_std_string_string_serialize_deserialize]
//         start: std::primitive::i32,
//         #[eo_to_std_string_string_serialize_deserialize]
//         end: std::primitive::i32,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ExcludedStartMoreThanIncludedEnd {
//         #[eo_to_std_string_string_serialize_deserialize]
//         start: std::primitive::i32,
//         #[eo_to_std_string_string_serialize_deserialize]
//         end: std::primitive::i32,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ExcludedStartMoreThanExcludedEnd {
//         #[eo_to_std_string_string_serialize_deserialize]
//         start: std::primitive::i32,
//         #[eo_to_std_string_string_serialize_deserialize]
//         end: std::primitive::i32,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     IncludedEndCannotBeStdPrimitiveI32Max {
//         #[eo_to_std_string_string_serialize_deserialize]
//         end: std::primitive::i32,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//     pub fn try_new(value: sqlx::postgres::types::PgRange<std::primitive::i32>) -> Result<Self, SqlxPostgresTypesPgRangeStdPrimitiveI32TryNewErrorNamed> {
//         let (start, end) = match (&value.start, &value.end) {
//             (std::ops::Bound::Included(start), std::ops::Bound::Included(end)) => {
//                 if start > end {
//                     return Err(SqlxPostgresTypesPgRangeStdPrimitiveI32TryNewErrorNamed::IncludedStartMoreThanIncludedEnd {
//                         start: *start,
//                         end: *end,
//                         code_occurence: error_occurence_lib::code_occurence!(),
//                     });
//                 }
//                 if *end == std::primitive::i32::MAX {
//                     return Err(SqlxPostgresTypesPgRangeStdPrimitiveI32TryNewErrorNamed::IncludedEndCannotBeStdPrimitiveI32Max {
//                         end: *end,
//                         code_occurence: error_occurence_lib::code_occurence!(),
//                     });
//                 }
//                 (
//                     std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                     std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                 )
//             },
//             (std::ops::Bound::Included(start), std::ops::Bound::Excluded(end)) => {
//                 if start > end {
//                     return Err(SqlxPostgresTypesPgRangeStdPrimitiveI32TryNewErrorNamed::IncludedStartMoreThanExcludedEnd {
//                         start: *start,
//                         end: *end,
//                         code_occurence: error_occurence_lib::code_occurence!(),
//                     });
//                 }
//                 (
//                     std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                     std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                 )
//             },
//             (std::ops::Bound::Included(start), std::ops::Bound::Unbounded) => {
//                 (
//                     std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                     std::ops::Bound::Unbounded
//                 )
//             },
//             (std::ops::Bound::Excluded(start), std::ops::Bound::Included(end)) => {
//                 if start > end {
//                     return Err(SqlxPostgresTypesPgRangeStdPrimitiveI32TryNewErrorNamed::ExcludedStartMoreThanIncludedEnd {
//                         start: *start,
//                         end: *end,
//                         code_occurence: error_occurence_lib::code_occurence!(),
//                     });
//                 }
//                 if *end == std::primitive::i32::MAX {
//                     return Err(SqlxPostgresTypesPgRangeStdPrimitiveI32TryNewErrorNamed::IncludedEndCannotBeStdPrimitiveI32Max {
//                         end: *end,
//                         code_occurence: error_occurence_lib::code_occurence!(),
//                     });
//                 }
//                 (
//                     std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                     std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                 )
//             },
//             (std::ops::Bound::Excluded(start), std::ops::Bound::Excluded(end)) => {
//                 if start > end {
//                     return Err(SqlxPostgresTypesPgRangeStdPrimitiveI32TryNewErrorNamed::ExcludedStartMoreThanExcludedEnd {
//                         start: *start,
//                         end: *end,
//                         code_occurence: error_occurence_lib::code_occurence!(),
//                     });
//                 }
//                 (
//                     std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                     std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                 )
//             },
//             (std::ops::Bound::Excluded(start), std::ops::Bound::Unbounded) => {
//                 (
//                     std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                     std::ops::Bound::Unbounded
//                 )
//             },
//             (std::ops::Bound::Unbounded, std::ops::Bound::Included(end)) => {
//                 if *end == std::primitive::i32::MAX {
//                     return Err(SqlxPostgresTypesPgRangeStdPrimitiveI32TryNewErrorNamed::IncludedEndCannotBeStdPrimitiveI32Max {
//                         end: *end,
//                         code_occurence: error_occurence_lib::code_occurence!(),
//                     });
//                 }
//                 (
//                     std::ops::Bound::Unbounded,
//                     std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                 )
//             },
//             (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(end)) => {
//                 (
//                     std::ops::Bound::Unbounded,
//                     std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                 )
//             },
//             (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => {
//                 (
//                     std::ops::Bound::Unbounded,
//                     std::ops::Bound::Unbounded
//                 )
//             },
//         };
//         Ok(Self(sqlx::postgres::types::PgRange {
//             start,
//             end
//         }))
//     }
//     pub fn get(&self) -> &sqlx::postgres::types::PgRange<StdPrimitiveI32AsNotNullInt4Origin> {
//         &self.0
//     }
// }
// const _: () = {
//     #[allow(unused_extern_crates, clippy::useless_attribute)]
//     extern crate serde as _serde;
//     #[automatically_derived]
//     impl _serde::Serialize for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//         fn serialize<__S>(
//             &self,
//             __serializer: __S,
//         ) -> _serde::__private::Result<__S::Ok, __S::Error>
//         where
//             __S: _serde::Serializer,
//         {
//             let mut __serde_state = _serde::Serializer::serialize_struct(
//                 __serializer,
//                 "SqlxPostgresTypesPgRangeStdPrimitiveI32",
//                 false as usize + 1 + 1,
//             )?;
//             _serde::ser::SerializeStruct::serialize_field(
//                 &mut __serde_state,
//                 "start",
//                 &self.0.start,
//             )?;
//             _serde::ser::SerializeStruct::serialize_field(
//                 &mut __serde_state,
//                 "end",
//                 &self.0.end,
//             )?;
//             _serde::ser::SerializeStruct::end(__serde_state)
//         }
//     }
// };
// #[doc(hidden)]
// #[allow(
//     non_upper_case_globals,
//     unused_attributes,
//     unused_qualifications,
//     clippy::absolute_paths,
// )]
// const _: () = {
//     #[allow(unused_extern_crates, clippy::useless_attribute)]
//     extern crate serde as _serde;
//     #[automatically_derived]
//     impl<'de> _serde::Deserialize<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//         fn deserialize<__D>(
//             __deserializer: __D,
//         ) -> _serde::__private::Result<Self, __D::Error>
//         where
//             __D: _serde::Deserializer<'de>,
//         {
//             #[allow(non_camel_case_types)]
//             #[doc(hidden)]
//             enum __Field {
//                 __field0,
//                 __field1,
//                 __ignore,
//             }
//             #[doc(hidden)]
//             struct __FieldVisitor;
//             #[automatically_derived]
//             impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
//                 type Value = __Field;
//                 fn expecting(
//                     &self,
//                     __formatter: &mut _serde::__private::Formatter<'_>,
//                 ) -> _serde::__private::fmt::Result {
//                     _serde::__private::Formatter::write_str(
//                         __formatter,
//                         "field identifier",
//                     )
//                 }
//                 fn visit_u64<__E>(
//                     self,
//                     __value: u64,
//                 ) -> _serde::__private::Result<Self::Value, __E>
//                 where
//                     __E: _serde::de::Error,
//                 {
//                     match __value {
//                         0u64 => _serde::__private::Ok(__Field::__field0),
//                         1u64 => _serde::__private::Ok(__Field::__field1),
//                         _ => _serde::__private::Ok(__Field::__ignore),
//                     }
//                 }
//                 fn visit_str<__E>(
//                     self,
//                     __value: &str,
//                 ) -> _serde::__private::Result<Self::Value, __E>
//                 where
//                     __E: _serde::de::Error,
//                 {
//                     match __value {
//                         "start" => _serde::__private::Ok(__Field::__field0),
//                         "end" => _serde::__private::Ok(__Field::__field1),
//                         _ => _serde::__private::Ok(__Field::__ignore),
//                     }
//                 }
//                 fn visit_bytes<__E>(
//                     self,
//                     __value: &[u8],
//                 ) -> _serde::__private::Result<Self::Value, __E>
//                 where
//                     __E: _serde::de::Error,
//                 {
//                     match __value {
//                         b"start" => _serde::__private::Ok(__Field::__field0),
//                         b"end" => _serde::__private::Ok(__Field::__field1),
//                         _ => _serde::__private::Ok(__Field::__ignore),
//                     }
//                 }
//             }
//             #[automatically_derived]
//             impl<'de> _serde::Deserialize<'de> for __Field {
//                 #[inline]
//                 fn deserialize<__D>(
//                     __deserializer: __D,
//                 ) -> _serde::__private::Result<Self, __D::Error>
//                 where
//                     __D: _serde::Deserializer<'de>,
//                 {
//                     _serde::Deserializer::deserialize_identifier(
//                         __deserializer,
//                         __FieldVisitor,
//                     )
//                 }
//             }
//             #[doc(hidden)]
//             struct __Visitor<'de> {
//                 marker: _serde::__private::PhantomData<
//                     SqlxPostgresTypesPgRangeStdPrimitiveI32,
//                 >,
//                 lifetime: _serde::__private::PhantomData<&'de ()>,
//             }
//             #[automatically_derived]
//             impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
//                 type Value = SqlxPostgresTypesPgRangeStdPrimitiveI32;
//                 fn expecting(
//                     &self,
//                     __formatter: &mut _serde::__private::Formatter<'_>,
//                 ) -> _serde::__private::fmt::Result {
//                     _serde::__private::Formatter::write_str(
//                         __formatter,
//                         "struct SqlxPostgresTypesPgRangeStdPrimitiveI32",
//                     )
//                 }
//                 #[inline]
//                 fn visit_seq<__A>(
//                     self,
//                     mut __seq: __A,
//                 ) -> _serde::__private::Result<Self::Value, __A::Error>
//                 where
//                     __A: _serde::de::SeqAccess<'de>,
//                 {
//                     let __field0 = match _serde::de::SeqAccess::next_element::<
//                         std::ops::Bound<std::primitive::i32>,
//                     >(&mut __seq)? {
//                         _serde::__private::Some(__value) => __value,
//                         _serde::__private::None => {
//                             return _serde::__private::Err(
//                                 _serde::de::Error::invalid_length(
//                                     0usize,
//                                     &"struct SqlxPostgresTypesPgRangeStdPrimitiveI32 with 2 elements",
//                                 ),
//                             );
//                         }
//                     };
//                     let __field1 = match _serde::de::SeqAccess::next_element::<
//                         std::ops::Bound<std::primitive::i32>,
//                     >(&mut __seq)? {
//                         _serde::__private::Some(__value) => __value,
//                         _serde::__private::None => {
//                             return _serde::__private::Err(
//                                 _serde::de::Error::invalid_length(
//                                     1usize,
//                                     &"struct SqlxPostgresTypesPgRangeStdPrimitiveI32 with 2 elements",
//                                 ),
//                             );
//                         }
//                     };
//                     match SqlxPostgresTypesPgRangeStdPrimitiveI32::try_new(sqlx::postgres::types::PgRange {
//                         start: __field0,
//                         end: __field1
//                     }) {
//                         Ok(value) => _serde::__private::Ok(value),
//                         Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
//                     }
//                 }
//                 #[inline]
//                 fn visit_map<__A>(
//                     self,
//                     mut __map: __A,
//                 ) -> _serde::__private::Result<Self::Value, __A::Error>
//                 where
//                     __A: _serde::de::MapAccess<'de>,
//                 {
//                     let mut __field0: _serde::__private::Option<
//                         std::ops::Bound<std::primitive::i32>,
//                     > = _serde::__private::None;
//                     let mut __field1: _serde::__private::Option<
//                         std::ops::Bound<std::primitive::i32>,
//                     > = _serde::__private::None;
//                     while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
//                         __Field,
//                     >(&mut __map)? {
//                         match __key {
//                             __Field::__field0 => {
//                                 if _serde::__private::Option::is_some(&__field0) {
//                                     return _serde::__private::Err(
//                                         <__A::Error as _serde::de::Error>::duplicate_field("start"),
//                                     );
//                                 }
//                                 __field0 = _serde::__private::Some(
//                                     _serde::de::MapAccess::next_value::<
//                                         std::ops::Bound<std::primitive::i32>,
//                                     >(&mut __map)?,
//                                 );
//                             }
//                             __Field::__field1 => {
//                                 if _serde::__private::Option::is_some(&__field1) {
//                                     return _serde::__private::Err(
//                                         <__A::Error as _serde::de::Error>::duplicate_field("end"),
//                                     );
//                                 }
//                                 __field1 = _serde::__private::Some(
//                                     _serde::de::MapAccess::next_value::<
//                                         std::ops::Bound<std::primitive::i32>,
//                                     >(&mut __map)?,
//                                 );
//                             }
//                             _ => {
//                                 let _ = _serde::de::MapAccess::next_value::<
//                                     _serde::de::IgnoredAny,
//                                 >(&mut __map)?;
//                             }
//                         }
//                     }
//                     let __field0 = match __field0 {
//                         _serde::__private::Some(__field0) => __field0,
//                         _serde::__private::None => {
//                             _serde::__private::de::missing_field("start")?
//                         }
//                     };
//                     let __field1 = match __field1 {
//                         _serde::__private::Some(__field1) => __field1,
//                         _serde::__private::None => {
//                             _serde::__private::de::missing_field("end")?
//                         }
//                     };
//                     match SqlxPostgresTypesPgRangeStdPrimitiveI32::try_new(sqlx::postgres::types::PgRange {
//                         start: __field0,
//                         end: __field1
//                     }) {
//                         Ok(value) => _serde::__private::Ok(value),
//                         Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
//                     }
//                 }
//             }
//             #[doc(hidden)]
//             const FIELDS: &'static [&'static str] = &["start", "end"];
//             _serde::Deserializer::deserialize_struct(
//                 __deserializer,
//                 "SqlxPostgresTypesPgRangeStdPrimitiveI32",
//                 FIELDS,
//                 __Visitor {
//                     marker: _serde::__private::PhantomData::<
//                         SqlxPostgresTypesPgRangeStdPrimitiveI32,
//                     >,
//                     lifetime: _serde::__private::PhantomData,
//                 },
//             )
//         }
//     }
// };
// impl std::convert::Into<sqlx::postgres::types::PgRange<std::primitive::i32>> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//     fn into(self) -> sqlx::postgres::types::PgRange<std::primitive::i32> {
//         sqlx::postgres::types::PgRange {
//             start: match self.0.start {
//                 std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),//<StdPrimitiveI32AsNotNullInt4 as crate::PostgresqlType>::into_inner(value)
//                 std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
//                 std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
//             },
//             end: match self.0.end {
//                 std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
//                 std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
//                 std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
//             },
//         }
//     }
// }
// impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//     fn to_std_string_string(&self) -> std::string::String {
//         self.0.to_string()
//     }
// }
// impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self(sqlx::postgres::types::PgRange {
//             start: std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(0)),
//             end: std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(10)),
//         })
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self({
//                 let (start, end) = match (&value.start, &value.end) {
//                     (std::ops::Bound::Included(start), std::ops::Bound::Included(end)) => {
//                         (
//                             std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                             std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                         )
//                     },
//                     (std::ops::Bound::Included(start), std::ops::Bound::Excluded(end)) => {
//                         (
//                             std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                             std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                         )
//                     },
//                     (std::ops::Bound::Included(start), std::ops::Bound::Unbounded) => {
//                         (
//                             std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                             std::ops::Bound::Unbounded
//                         )
//                     },
//                     (std::ops::Bound::Excluded(start), std::ops::Bound::Included(end)) => {
//                         (
//                             std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                             std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                         )
//                     },
//                     (std::ops::Bound::Excluded(start), std::ops::Bound::Excluded(end)) => {
//                         (
//                             std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                             std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                         )
//                     },
//                     (std::ops::Bound::Excluded(start), std::ops::Bound::Unbounded) => {
//                         (
//                             std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*start)),
//                             std::ops::Bound::Unbounded
//                         )
//                     },
//                     (std::ops::Bound::Unbounded, std::ops::Bound::Included(end)) => {
//                         (
//                             std::ops::Bound::Unbounded,
//                             std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                         )
//                     },
//                     (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(end)) => {
//                         (
//                             std::ops::Bound::Unbounded,
//                             std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(*end))
//                         )
//                     },
//                     (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => {
//                         (
//                             std::ops::Bound::Unbounded,
//                             std::ops::Bound::Unbounded
//                         )
//                     },
//                 };
//                 sqlx::postgres::types::PgRange {
//                     start,
//                     end
//                 }
//             })),
//             Err(error) => Err(error),
//         }
//     }
// }
// impl sqlx::postgres::PgHasArrayType for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//     fn array_type_info() -> sqlx::postgres::PgTypeInfo {
//         <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::postgres::PgHasArrayType>::array_type_info()
//     }
// }


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


#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRange;
#[derive(Debug, Clone, PartialEq)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin(sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>);
//here
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOriginTryNewErrorNamed {
    Start {
        #[eo_error_occurence]
        error: SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOriginTryNewErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    End {
        #[eo_error_occurence]
        error: SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOriginTryNewErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
    //here
    pub fn try_new(value: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>) -> Result<Self, SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOriginTryNewErrorNamed> {
        let value = sqlx::postgres::types::PgRange {
            start: match value.start {
                std::ops::Bound::Included(value) => {
                    match SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::try_new(value) {
                        Ok(value) => std::ops::Bound::Included(value.0),
                        Err(error) => {
                            return Err(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOriginTryNewErrorNamed::Start {
                                error,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                },
                std::ops::Bound::Excluded(value) => {
                    match SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::try_new(value) {
                        Ok(value) => std::ops::Bound::Excluded(value.0),
                        Err(error) => {
                            return Err(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOriginTryNewErrorNamed::Start {
                                error,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                },
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
            end: match value.end {
                std::ops::Bound::Included(value) => {
                    match SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::try_new(value) {
                        Ok(value) => std::ops::Bound::Included(value.0),
                        Err(error) => {
                            return Err(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOriginTryNewErrorNamed::End {
                                error,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                },
                std::ops::Bound::Excluded(value) => {
                    match SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::try_new(value) {
                        Ok(value) => std::ops::Bound::Excluded(value.0),
                        Err(error) => {
                            return Err(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOriginTryNewErrorNamed::End {
                                error,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                },
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        };
        Ok(Self(value))
    }
    //here
    fn new_for_deserialize(
        start: std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>,
        end: std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>
    ) -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: match start {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
            end: match end {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            }
        })
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>) -> Self {
        //here
        Self::try_new(value).unwrap()
    }
}
//here
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin", false as std::primitive::usize + 1 + 1)?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &match self.0.start {
                    std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::try_new(value).unwrap()),
                    std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::try_new(value).unwrap()),
                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                }
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &match self.0.end {
                    std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::try_new(value).unwrap()),
                    std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::try_new(value).unwrap()),
                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                }
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
//here
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "start" => _serde::__private::Ok(__Field::__field0),
                        "end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
                where
                    __E: serde::de::Error,
                {
                    match __value {
                        b"start" => serde::__private::Ok(__Field::__field0),
                        b"end" => serde::__private::Ok(__Field::__field1),
                        _ => serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin>,
                lifetime: serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin;
                fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                    serde::__private::Formatter::write_str(__f, "struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin with 2 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin with 2 elements"));
                        }
                    };
                    //here 
                    serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin::new_for_deserialize(__field0, __field1))
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: serde::de::MapAccess<'de>,
                {
                    let mut __field0: serde::__private::Option<std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>> = serde::__private::None;
                    while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("\"start\""));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("\"end\""));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>>(&mut __map)?);
                            }
                            _ => {
                                let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("\"start\"")?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde::__private::de::missing_field("\"end\"")?,
                    };
                    //here 
                    serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin::new_for_deserialize(__field0, __field1))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRange",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
//here
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(
                <SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin as crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element().0
            ),
            end: std::ops::Bound::Excluded(
                <SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin as crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element().0
            ),
        })
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime> as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} tsrange not null")
    }
}
pub type SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin;
pub type SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeCreate = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin>),
    FindRangesWithinGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementFindRangesWithinGivenRange<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration>),
    FindRangesThatFullyContainTheGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementFindRangesThatFullyContainTheGivenRange<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration>),
    StrictlyToLeftOfRange(crate::where_element_filters::PostgresqlTypeWhereElementStrictlyToLeftOfRange<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration>),
    StrictlyToRightOfRange(crate::where_element_filters::PostgresqlTypeWhereElementStrictlyToRightOfRange<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration>),
    IncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementIncludedLowerBound<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration>),
    ExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementExcludedUpperBound<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration>),
    GreaterThanIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanIncludedLowerBound<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration>),
    GreaterThanExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanExcludedUpperBound<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration>),
    OverlapWithRange(crate::where_element_filters::PostgresqlTypeWhereElementOverlapWithRange<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration>),
    AdjacentWithRange(crate::where_element_filters::PostgresqlTypeWhereElementAdjacentWithRange<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration>),
    RangeLength(crate::where_element_filters::PostgresqlTypeWhereElementRangeLength),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::FindRangesWithinGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::FindRangesThatFullyContainTheGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::StrictlyToLeftOfRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::StrictlyToRightOfRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::IncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::ExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::OverlapWithRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AdjacentWithRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::RangeLength(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::FindRangesWithinGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::FindRangesThatFullyContainTheGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::StrictlyToLeftOfRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::StrictlyToRightOfRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::IncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::ExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::OverlapWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::AdjacentWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::RangeLength(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::FindRangesWithinGivenRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::FindRangesThatFullyContainTheGivenRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::StrictlyToLeftOfRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::StrictlyToRightOfRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::IncludedLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ExcludedUpperBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThanIncludedLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThanExcludedUpperBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::OverlapWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AdjacentWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::RangeLength(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeRead(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeRead {
    //here
    pub fn try_new(value: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>) -> Result<Self, SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOriginTryNewErrorNamed> {
        match SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin::try_new(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>) -> Self {
        Self(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeReadInner = sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>;
pub type SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeUpdate = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeOrigin;
impl crate::PostgresqlType for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRange {
    type TableTypeDeclaration = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeTableTypeDeclaration;
    type Create = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeCreate;
    fn create_query_part(_: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                acc.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(acc)
    }
    fn create_query_bind(value: Self::Create, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(value);
        query
    }
    type Select = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeSelect;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeWhereElement;
    type Read = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeRead;
    type ReadInner = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeReadInner;
    //here
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        sqlx::postgres::types::PgRange {
            start: match value.0.0.start {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(value),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
            end: match value.0.0.end {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(value),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        }
    }
    type Update = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRangeUpdate;
    fn update_query_part(_: &Self::Update, _: &std::primitive::str, _: &std::primitive::str, _: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                acc.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(acc)
    }
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(value);
        query
    }
}
impl crate::tests::PostgresqlTypeTestCases for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRange {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        vec![]
    }
}