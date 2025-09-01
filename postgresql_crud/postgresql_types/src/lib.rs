generate_postgresql_types::generate_postgresql_types!("All");
// generate_postgresql_types::generate_postgresql_types!({
//     "Concrete": [
//         {
//             "postgresql_type": "StdPrimitiveI16AsInt2",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI32AsInt4",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI32AsInt4",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI32AsInt4",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI32AsInt4",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI32AsInt4",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI32AsInt4",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI64AsInt8",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI64AsInt8",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI64AsInt8",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI64AsInt8",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI64AsInt8",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI64AsInt8",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF32AsFloat4",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF32AsFloat4",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF32AsFloat4",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF32AsFloat4",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF32AsFloat4",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF32AsFloat4",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF64AsFloat8",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF64AsFloat8",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF64AsFloat8",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF64AsFloat8",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF64AsFloat8",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveF64AsFloat8",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsSmallSerialInitializedByPostgresql",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI32AsSerialInitializedByPostgresql",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI64AsBigSerialInitializedByPostgresql",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgMoneyAsMoney",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveBoolAsBool",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveBoolAsBool",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveBoolAsBool",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveBoolAsBool",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveBoolAsBool",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveBoolAsBool",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdStringStringAsText",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdStringStringAsText",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdStringStringAsText",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdStringStringAsText",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdStringStringAsText",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdStringStringAsText",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         {
//             "postgresql_type": "SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         },
//         {
//             "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         },
//         {
//             "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_type_pattern": "Standart"
//         },
//         {
//             "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesIpnetworkIpNetworkAsInet",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // }
//     ]
// });

pub trait PostgresqlTypePrimaryKey {
    type PrimaryKey;
}

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
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        self.0.query_bind(query)
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PaginationStartsWithOne {
    #[inline]
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(
            postgresql_crud_common::PaginationBase::new_unchecked(
                postgresql_crud_common::DEFAULT_PAGINATION_LIMIT,
                1
            )
        )
    }
}