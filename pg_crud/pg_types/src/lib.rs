//todo
pub use pg_types_common::{PaginationStartsWithOne, PaginationStartsWithOneTryNewErrorNamed};

gen_pg_types::gen_pg_types!({
    "pg_table_columns_content_write_into_pg_table_columns_using_pg_types": "False",
    "whole_content_write_into_gen_pg_types": "False",
    "variant":
    "All"
    // {
    //     "Concrete": [
    //         {
    //             "pg_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI16AsInt2",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI32AsInt4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI64AsInt8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF32AsFloat4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveF64AsFloat8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI16AsSmallSerialInitializedByPg",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI32AsSerialInitializedByPg",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveI64AsBigSerialInitializedByPg",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdPrimitiveBoolAsBool",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdStringStringAsText",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecStdPrimitiveU8AsBytea",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidV4InitializedByPg",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitializedByClient",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI32AsInt4Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeStdPrimitiveI64AsInt8Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         }
    //     ]
    // }
});
