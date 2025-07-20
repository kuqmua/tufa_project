generate_postgresql_types::generate_postgresql_types!("All");
// generate_postgresql_types::generate_postgresql_types!({
//     "Concrete": [
//         // {
//         //     "postgresql_type": "StdPrimitiveI64AsInt8",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         {
//             "postgresql_type": "StdStringStringAsText",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         }
//         ,
//         // {
//         //     "postgresql_type": "StdPrimitiveI32AsInt4",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
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
//         //             "dimension1_not_null_or_nullable": "NotNull"
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
//         //             "dimension1_not_null_or_nullable": "NotNull"
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
//         //     "postgresql_type": "StdPrimitiveI64AsBigSerialInitializedByPostgresql",
//         //     "not_null_or_nullable": "NotNull",
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
//         //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesTimeDateAsDate",
//         //     "not_null_or_nullable": "NotNull",
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
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
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
//         //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
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
//         //     "not_null_or_nullable": "NotNull",
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
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         {
//             "postgresql_type": "SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         }
//     ]
// });
