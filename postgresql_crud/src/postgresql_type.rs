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
        {
            "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        },
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