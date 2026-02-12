//todo
pub use postgres_types_common::{PaginationStartsWithOne, PaginationStartsWithOneTryNewErrorNamed};

gen_postgres_types::gen_postgres_types!({
    "postgres_table_columns_content_write_into_postgres_table_columns_using_postgres_types": "False",
    "whole_content_write_into_gen_postgres_types": "False",
    "variant":
    "All"
    // {
    //     "Concrete": [
    //         {
    //             "postgres_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI16AsSmallSerialInitializedByPostgres",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI32AsSerialInitializedByPostgres",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveI64AsBigSerialInitializedByPostgres",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesUuidUuidAsUuidV4InitializedByPostgres",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": "Standart"
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "NotNull",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "postgres_type": "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "Nullable",
    //             "postgres_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         }
    //     ]
    // }
});
