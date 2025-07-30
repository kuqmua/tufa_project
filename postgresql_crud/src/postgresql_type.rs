// generate_postgresql_types::generate_postgresql_types!("All");
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
//         //     "postgresql_type": "StdPrimitiveI64AsInt8",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         {
//             "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         },
//         {
//             "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         },
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
//         {
//             "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         },
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
//         ,
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
//         {
//             "postgresql_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         }
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
pub struct StdPrimitiveI16AsNotNullInt2;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI16AsNotNullInt2Origin(std::primitive::i16);
impl StdPrimitiveI16AsNotNullInt2Origin {
    pub fn new(value: std::primitive::i16) -> Self {
        Self(value)
    }
    pub fn new_or_try_new_unwraped_for_test(value: std::primitive::i16) -> Self {
        Self::new(value)
    }
}
impl std::fmt::Display for StdPrimitiveI16AsNotNullInt2Origin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI16AsNotNullInt2Origin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI16AsNotNullInt2Origin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI16AsNotNullInt2Origin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i16 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i16 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI16AsNotNullInt2Origin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI16AsNotNullInt2Origin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::primitive::i16 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for StdPrimitiveI16AsNotNullInt2Origin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <std::primitive::i16 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl StdPrimitiveI16AsNotNullInt2Origin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} int2 not null")
    }
}
pub type StdPrimitiveI16AsNotNullInt2TableTypeDeclaration = StdPrimitiveI16AsNotNullInt2Origin;
pub type StdPrimitiveI16AsNotNullInt2Create = StdPrimitiveI16AsNotNullInt2Origin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI16AsNotNullInt2Select;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI16AsNotNullInt2Select {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum StdPrimitiveI16AsNotNullInt2WhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<StdPrimitiveI16AsNotNullInt2Origin>),
    GreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThan<StdPrimitiveI16AsNotNullInt2TableTypeDeclaration>),
    Between(crate::where_element_filters::PostgresqlTypeWhereElementBetween<StdPrimitiveI16AsNotNullInt2TableTypeDeclaration>),
    In(crate::where_element_filters::PostgresqlTypeWhereElementIn<StdPrimitiveI16AsNotNullInt2TableTypeDeclaration>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for StdPrimitiveI16AsNotNullInt2WhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::In(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::In(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI16AsNotNullInt2WhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI16AsNotNullInt2WhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::In(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI16AsNotNullInt2Read(StdPrimitiveI16AsNotNullInt2Origin);
impl StdPrimitiveI16AsNotNullInt2Read {
    pub fn new(value: std::primitive::i16) -> Self {
        Self(StdPrimitiveI16AsNotNullInt2Origin::new(value))
    }
    pub fn new_or_try_new_unwraped_for_test(value: std::primitive::i16) -> Self {
        Self(StdPrimitiveI16AsNotNullInt2Origin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI16AsNotNullInt2Read {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI16AsNotNullInt2Read {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI16AsNotNullInt2Read {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI16AsNotNullInt2Read {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <StdPrimitiveI16AsNotNullInt2Origin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI16AsNotNullInt2Read {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <StdPrimitiveI16AsNotNullInt2Origin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <StdPrimitiveI16AsNotNullInt2Origin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type StdPrimitiveI16AsNotNullInt2ReadInner = std::primitive::i16;
pub type StdPrimitiveI16AsNotNullInt2Update = StdPrimitiveI16AsNotNullInt2Origin;
impl crate::PostgresqlType for StdPrimitiveI16AsNotNullInt2 {
    type TableTypeDeclaration = StdPrimitiveI16AsNotNullInt2TableTypeDeclaration;
    type Create = StdPrimitiveI16AsNotNullInt2Create;
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
    type Select = StdPrimitiveI16AsNotNullInt2Select;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = StdPrimitiveI16AsNotNullInt2WhereElement;
    type Read = StdPrimitiveI16AsNotNullInt2Read;
    type ReadInner = StdPrimitiveI16AsNotNullInt2ReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = StdPrimitiveI16AsNotNullInt2Update;
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
impl crate::tests::PostgresqlTypeTestCases for StdPrimitiveI16AsNotNullInt2 {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        vec![std::primitive::i16::MIN, 0, std::primitive::i16::MAX]
    }
}
#[derive(Debug)]
pub struct SqlxTypesChronoNaiveDateAsNotNullDate;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize)]
pub struct SqlxTypesChronoNaiveDateAsNotNullDateOrigin(sqlx::types::chrono::NaiveDate);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum SqlxTypesChronoNaiveDateAsNotNullDateOriginTryNewErrorNamed {
    EarlierDateNotSupported {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::string::String,
        #[eo_to_std_string_string_serialize_deserialize]
        earliest_supported_date: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
    pub fn try_new(value: sqlx::types::chrono::NaiveDate) -> Result<Self, SqlxTypesChronoNaiveDateAsNotNullDateOriginTryNewErrorNamed> {
        let earliest_supported_date = sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 1, 1).unwrap();
        if value > earliest_supported_date {
            Ok(Self(value))
        } else {
            Err(SqlxTypesChronoNaiveDateAsNotNullDateOriginTryNewErrorNamed::EarlierDateNotSupported {
                value: value.to_string(),
                earliest_supported_date: earliest_supported_date.to_string(),
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::chrono::NaiveDate) -> Self {
        Self::try_new(value).unwrap()
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SqlxTypesChronoNaiveDateAsNotNullDateOrigin>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesChronoNaiveDateAsNotNullDateOrigin;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct SqlxTypesChronoNaiveDateAsNotNullDateOrigin")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: sqlx::types::chrono::NaiveDate = <sqlx::types::chrono::NaiveDate as _serde::Deserialize>::deserialize(__e)?;
                    match SqlxTypesChronoNaiveDateAsNotNullDateOrigin::try_new(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<sqlx::types::chrono::NaiveDate>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct SqlxTypesChronoNaiveDateAsNotNullDateOrigin with 1 element"));
                        }
                    };
                    match SqlxTypesChronoNaiveDateAsNotNullDateOrigin::try_new(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "SqlxTypesChronoNaiveDateAsNotNullDateOrigin",
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxTypesChronoNaiveDateAsNotNullDateOrigin>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveDate as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::NaiveDate as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::chrono::NaiveDate as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => match Self::try_new(value) {
                Ok(value) => Ok(value),
                Err(error) => Err(std::boxed::Box::new(error)),
            },
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::chrono::NaiveDate as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} date not null")
    }
}
pub type SqlxTypesChronoNaiveDateAsNotNullDateTableTypeDeclaration = SqlxTypesChronoNaiveDateAsNotNullDateOrigin;
pub type SqlxTypesChronoNaiveDateAsNotNullDateCreate = SqlxTypesChronoNaiveDateAsNotNullDateOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoNaiveDateAsNotNullDateSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveDateAsNotNullDateSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxTypesChronoNaiveDateAsNotNullDateWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxTypesChronoNaiveDateAsNotNullDateOrigin>),
    GreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThan<SqlxTypesChronoNaiveDateAsNotNullDateTableTypeDeclaration>),
    Between(crate::where_element_filters::PostgresqlTypeWhereElementBetween<SqlxTypesChronoNaiveDateAsNotNullDateTableTypeDeclaration>),
    CurrentDate(crate::where_element_filters::PostgresqlTypeWhereElementCurrentDate),
    GreaterThanCurrentDate(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanCurrentDate),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxTypesChronoNaiveDateAsNotNullDateWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CurrentDate(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThanCurrentDate(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::CurrentDate(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThanCurrentDate(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveDateAsNotNullDateWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveDateAsNotNullDateWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CurrentDate(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThanCurrentDate(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoNaiveDateAsNotNullDateRead(SqlxTypesChronoNaiveDateAsNotNullDateOrigin);
impl SqlxTypesChronoNaiveDateAsNotNullDateRead {
    pub fn try_new(value: sqlx::types::chrono::NaiveDate) -> Result<Self, SqlxTypesChronoNaiveDateAsNotNullDateOriginTryNewErrorNamed> {
        match SqlxTypesChronoNaiveDateAsNotNullDateOrigin::try_new(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::chrono::NaiveDate) -> Self {
        Self(SqlxTypesChronoNaiveDateAsNotNullDateOrigin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveDateAsNotNullDateRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveDateAsNotNullDateRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateAsNotNullDateRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateAsNotNullDateRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxTypesChronoNaiveDateAsNotNullDateOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveDateAsNotNullDateRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxTypesChronoNaiveDateAsNotNullDateOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <SqlxTypesChronoNaiveDateAsNotNullDateOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type SqlxTypesChronoNaiveDateAsNotNullDateReadInner = sqlx::types::chrono::NaiveDate;
pub type SqlxTypesChronoNaiveDateAsNotNullDateUpdate = SqlxTypesChronoNaiveDateAsNotNullDateOrigin;
impl crate::PostgresqlType for SqlxTypesChronoNaiveDateAsNotNullDate {
    type TableTypeDeclaration = SqlxTypesChronoNaiveDateAsNotNullDateTableTypeDeclaration;
    type Create = SqlxTypesChronoNaiveDateAsNotNullDateCreate;
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
    type Select = SqlxTypesChronoNaiveDateAsNotNullDateSelect;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxTypesChronoNaiveDateAsNotNullDateWhereElement;
    type Read = SqlxTypesChronoNaiveDateAsNotNullDateRead;
    type ReadInner = SqlxTypesChronoNaiveDateAsNotNullDateReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = SqlxTypesChronoNaiveDateAsNotNullDateUpdate;
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
impl crate::tests::PostgresqlTypeTestCases for SqlxTypesChronoNaiveDateAsNotNullDate {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        vec![sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31).unwrap(), sqlx::types::chrono::NaiveDate::MAX]
    }
}
#[derive(Debug)]
pub struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin(sqlx::types::chrono::NaiveDateTime);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOriginTryNewErrorNamed {
    NaiveDate {
        #[eo_error_occurence]
        error: SqlxTypesChronoNaiveDateAsNotNullDateOriginTryNewErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NaiveTime {
        #[eo_error_occurence]
        error: SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
    pub fn try_new(value: sqlx::types::chrono::NaiveDateTime) -> Result<Self, SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOriginTryNewErrorNamed> {
        let date = match SqlxTypesChronoNaiveDateAsNotNullDateOrigin::try_new(value.date()) {
            Ok(value) => value,
            Err(error) => {
                return Err(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOriginTryNewErrorNamed::NaiveDate {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        let time = match SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::try_new(value.time()) {
            Ok(value) => value,
            Err(error) => {
                return Err(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOriginTryNewErrorNamed::NaiveTime {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        Ok(Self(sqlx::types::chrono::NaiveDateTime::new(date.0, time.0)))
    }
    fn new_for_deserialize(date: SqlxTypesChronoNaiveDateAsNotNullDateOrigin, time: SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin) -> Self {
        Self(sqlx::types::chrono::NaiveDateTime::new(date.0, time.0))
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::chrono::NaiveDateTime) -> Self {
        Self::try_new(value).unwrap()
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin", false as usize + 1 + 1)?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "date", &SqlxTypesChronoNaiveDateAsNotNullDateOrigin::try_new(self.0.date()).unwrap())?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "time", &SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::try_new(self.0.time()).unwrap())?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
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
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "date" => _serde::__private::Ok(__Field::__field0),
                        "time" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"date" => _serde::__private::Ok(__Field::__field0),
                        b"time" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
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
                marker: _serde::__private::PhantomData<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<SqlxTypesChronoNaiveDateAsNotNullDateOrigin>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin with 2 elements"));
                        }
                    };
                    _serde::__private::Ok(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::new_for_deserialize(__field0, __field1))
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<SqlxTypesChronoNaiveDateAsNotNullDateOrigin> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("date"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<SqlxTypesChronoNaiveDateAsNotNullDateOrigin>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("time"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("date")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("time")?,
                    };
                    _serde::__private::Ok(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::new_for_deserialize(__field0, __field1))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["date", "time"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(sqlx::types::chrono::NaiveDateTime::new(
            <SqlxTypesChronoNaiveDateAsNotNullDateOrigin as crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element().0,
            <SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin as crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element().0,
        ))
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveDateTime as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::NaiveDateTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::chrono::NaiveDateTime as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::chrono::NaiveDateTime as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} timestamp not null")
    }
}
pub type SqlxTypesChronoNaiveDateTimeAsNotNullTimestampTableTypeDeclaration = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin;
pub type SqlxTypesChronoNaiveDateTimeAsNotNullTimestampCreate = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestampSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxTypesChronoNaiveDateTimeAsNotNullTimestampWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin>),
    GreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThan<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampTableTypeDeclaration>),
    Between(crate::where_element_filters::PostgresqlTypeWhereElementBetween<SqlxTypesChronoNaiveDateTimeAsNotNullTimestampTableTypeDeclaration>),
    CurrentTimestamp(crate::where_element_filters::PostgresqlTypeWhereElementCurrentTimestamp),
    GreaterThanCurrentTimestamp(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanCurrentTimestamp),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CurrentTimestamp(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThanCurrentTimestamp(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::CurrentTimestamp(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThanCurrentTimestamp(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CurrentTimestamp(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThanCurrentTimestamp(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestampRead(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin);
impl SqlxTypesChronoNaiveDateTimeAsNotNullTimestampRead {
    pub fn try_new(value: sqlx::types::chrono::NaiveDateTime) -> Result<Self, SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOriginTryNewErrorNamed> {
        match SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::try_new(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::chrono::NaiveDateTime) -> Self {
        Self(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type SqlxTypesChronoNaiveDateTimeAsNotNullTimestampReadInner = sqlx::types::chrono::NaiveDateTime;
pub type SqlxTypesChronoNaiveDateTimeAsNotNullTimestampUpdate = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin;
impl crate::PostgresqlType for SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp {
    type TableTypeDeclaration = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampTableTypeDeclaration;
    type Create = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampCreate;
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
    type Select = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampSelect;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampWhereElement;
    type Read = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampRead;
    type ReadInner = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampUpdate;
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
impl crate::tests::PostgresqlTypeTestCases for SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        vec![
            sqlx::types::chrono::NaiveDateTime::new(sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31).unwrap(), sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).unwrap()),
            sqlx::types::chrono::NaiveDateTime::new(sqlx::types::chrono::NaiveDate::MAX, sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_999).unwrap()),
        ]
    }
}
#[derive(Debug)]
pub struct SqlxTypesChronoNaiveTimeAsNotNullTime;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin(sqlx::types::chrono::NaiveTime);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewErrorNamed {
    NanosecondPrecisionIsNotSupported {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewForDeserializeErrorNamed {
    InvalidHourOrMinuteOrSecondOrMicrosecond {
        #[eo_to_std_string_string_serialize_deserialize]
        hour: std::primitive::u32,
        #[eo_to_std_string_string_serialize_deserialize]
        min: std::primitive::u32,
        #[eo_to_std_string_string_serialize_deserialize]
        sec: std::primitive::u32,
        #[eo_to_std_string_string_serialize_deserialize]
        micro: std::primitive::u32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NanosecondPrecisionIsNotSupported {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    pub fn try_new(value: sqlx::types::chrono::NaiveTime) -> Result<Self, SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewErrorNamed> {
        if <sqlx::types::chrono::NaiveTime as chrono::Timelike>::nanosecond(&value) % 1000 != 0 {
            return Err(SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewErrorNamed::NanosecondPrecisionIsNotSupported {
                value: value.to_string(),
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        Ok(Self(value))
    }
    fn try_new_for_deserialize(hour: std::primitive::u32, min: std::primitive::u32, sec: std::primitive::u32, micro: std::primitive::u32) -> Result<Self, SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewForDeserializeErrorNamed> {
        match sqlx::types::chrono::NaiveTime::from_hms_micro_opt(hour, min, sec, micro) {
            Some(value) => {
                if <sqlx::types::chrono::NaiveTime as chrono::Timelike>::nanosecond(&value) % 1000 != 0 {
                    return Err(SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewForDeserializeErrorNamed::NanosecondPrecisionIsNotSupported {
                        value: value.to_string(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
                Ok(Self(value))
            }
            None => Err(SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewForDeserializeErrorNamed::InvalidHourOrMinuteOrSecondOrMicrosecond {
                hour,
                min,
                sec,
                micro,
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        }
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::chrono::NaiveTime) -> Self {
        Self::try_new(value).unwrap()
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin", false as usize + 1 + 1 + 1 + 1)?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "hour", &<sqlx::types::chrono::NaiveTime as chrono::Timelike>::hour(&self.0))?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "min", &<sqlx::types::chrono::NaiveTime as chrono::Timelike>::minute(&self.0))?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "sec", &<sqlx::types::chrono::NaiveTime as chrono::Timelike>::second(&self.0))?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "micro", &(<sqlx::types::chrono::NaiveTime as chrono::Timelike>::nanosecond(&self.0) / 1000))?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "hour" => _serde::__private::Ok(__Field::__field0),
                        "min" => _serde::__private::Ok(__Field::__field1),
                        "sec" => _serde::__private::Ok(__Field::__field2),
                        "micro" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"hour" => _serde::__private::Ok(__Field::__field0),
                        b"min" => _serde::__private::Ok(__Field::__field1),
                        b"sec" => _serde::__private::Ok(__Field::__field2),
                        b"micro" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
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
                marker: _serde::__private::PhantomData<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::primitive::u32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::primitive::u32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::primitive::u32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements"));
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<std::primitive::u32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(3usize, &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements"));
                        }
                    };
                    match SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::try_new_for_deserialize(__field0, __field1, __field2, __field3) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<std::primitive::u32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::primitive::u32> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::primitive::u32> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<std::primitive::u32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("hour"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::u32>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("min"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::u32>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("sec"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::u32>(&mut __map)?);
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("micro"));
                                }
                                __field3 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::u32>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("hour")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("min")?,
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("sec")?,
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => _serde::__private::de::missing_field("micro")?,
                    };
                    match SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::try_new_for_deserialize(__field0, __field1, __field2, __field3) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["hour", "min", "sec", "micro"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveTime as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::NaiveTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::chrono::NaiveTime as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::chrono::NaiveTime as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} time not null")
    }
}
pub type SqlxTypesChronoNaiveTimeAsNotNullTimeTableTypeDeclaration = SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin;
pub type SqlxTypesChronoNaiveTimeAsNotNullTimeCreate = SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoNaiveTimeAsNotNullTimeSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveTimeAsNotNullTimeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxTypesChronoNaiveTimeAsNotNullTimeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin>),
    GreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThan<SqlxTypesChronoNaiveTimeAsNotNullTimeTableTypeDeclaration>),
    Between(crate::where_element_filters::PostgresqlTypeWhereElementBetween<SqlxTypesChronoNaiveTimeAsNotNullTimeTableTypeDeclaration>),
    CurrentTime(crate::where_element_filters::PostgresqlTypeWhereElementCurrentTime),
    GreaterThanCurrentTime(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanCurrentTime),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxTypesChronoNaiveTimeAsNotNullTimeWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CurrentTime(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThanCurrentTime(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::CurrentTime(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThanCurrentTime(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveTimeAsNotNullTimeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveTimeAsNotNullTimeWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CurrentTime(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThanCurrentTime(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoNaiveTimeAsNotNullTimeRead(SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin);
impl SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    pub fn try_new(value: sqlx::types::chrono::NaiveTime) -> Result<Self, SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewErrorNamed> {
        match SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::try_new(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::chrono::NaiveTime) -> Self {
        Self(SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type SqlxTypesChronoNaiveTimeAsNotNullTimeReadInner = sqlx::types::chrono::NaiveTime;
pub type SqlxTypesChronoNaiveTimeAsNotNullTimeUpdate = SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin;
impl crate::PostgresqlType for SqlxTypesChronoNaiveTimeAsNotNullTime {
    type TableTypeDeclaration = SqlxTypesChronoNaiveTimeAsNotNullTimeTableTypeDeclaration;
    type Create = SqlxTypesChronoNaiveTimeAsNotNullTimeCreate;
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
    type Select = SqlxTypesChronoNaiveTimeAsNotNullTimeSelect;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxTypesChronoNaiveTimeAsNotNullTimeWhereElement;
    type Read = SqlxTypesChronoNaiveTimeAsNotNullTimeRead;
    type ReadInner = SqlxTypesChronoNaiveTimeAsNotNullTimeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = SqlxTypesChronoNaiveTimeAsNotNullTimeUpdate;
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
impl crate::tests::PostgresqlTypeTestCases for SqlxTypesChronoNaiveTimeAsNotNullTime {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        vec![sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).unwrap(), sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_999).unwrap()]
    }
}
#[derive(Debug)]
pub struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql;
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin(sqlx::types::uuid::Uuid);
impl SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    pub fn new(value: sqlx::types::uuid::Uuid) -> Self {
        Self(value)
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::uuid::Uuid) -> Self {
        Self::new(value)
    }
}
impl crate::IsStringEmpty for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn is_string_empty(&self) -> std::primitive::bool {
        self.0.to_string().is_empty()
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin", &self.0.to_string())
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: serde::__private::PhantomData<SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin>,
                lifetime: serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin;
                fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                    serde::__private::Formatter::write_str(__f, "struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: serde::Deserializer<'de>,
                {
                    let __field0 = <std::string::String as serde::Deserialize>::deserialize(__e)?;
                    serde::__private::Ok(SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin(match sqlx::types::uuid::Uuid::try_parse(&__field0) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(error));
                        }
                    }))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin with 1 elements"));
                        }
                    };
                    serde::__private::Ok(SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin(match sqlx::types::uuid::Uuid::try_parse(&__field0) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(error));
                        }
                    }))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin",
                __Visitor {
                    marker: serde::__private::PhantomData::<SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin>,
                    lifetime: serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::uuid::Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::uuid::Uuid as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::uuid::Uuid as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::uuid::Uuid as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} uuid not null {}", crate::maybe_primary_key(is_primary_key))
    }
}
impl std::convert::From<SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn from(value: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead) -> Self {
        Self::new(<SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as crate::PostgresqlType>::into_inner(value))
    }
}
pub type SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlCreate(());
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin>),
    RegularExpression(crate::where_element_filters::PostgresqlTypeWhereElementRegularExpression),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::RegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::RegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::RegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead(SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin);
impl SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    pub fn new(value: sqlx::types::uuid::Uuid) -> Self {
        Self(SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(value))
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::uuid::Uuid) -> Self {
        Self(SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, _: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("({} = ${})", column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self);
        query
    }
}
impl crate::PostgresqlTypePrimaryKey for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql {
    type PrimaryKey = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead;
}
pub type SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlReadInner = sqlx::types::uuid::Uuid;
pub type SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin;
impl crate::PostgresqlType for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql {
    type TableTypeDeclaration = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration;
    type Create = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlCreate;
    fn create_query_part(_: &Self::Create, _: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        Ok(std::string::String::from("uuid_generate_v4()"))
    }
    fn create_query_bind(_: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query
    }
    type Select = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlSelect;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement;
    type Read = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead;
    type ReadInner = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate;
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
impl crate::tests::PostgresqlTypeTestCases for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        vec![]
    }
}
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
