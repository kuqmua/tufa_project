generate_postgresql_types::generate_postgresql_types!("All");
// generate_postgresql_types::generate_postgresql_types!({
//     "Concrete": [
//         {
//             "postgresql_type": "StdPrimitiveI16AsInt2",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         },
//         {
//             "postgresql_type": "StdPrimitiveI16AsInt2",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_type_pattern": "Standart"
//         },
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
//         //     "postgresql_type": "StdPrimitiveI64AsInt8",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
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
//         }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
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
//         // }
//         // ,
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
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // }
//         // ,
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
//         // }
//         // ,
        
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
//         // }
//         // ,
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
//         // }
//         // ,
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