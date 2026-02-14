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
    //             "pg_type": "I16AsInt2",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsSmallSerialInitializedByPg",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I32AsSerialInitializedByPg",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I64AsBigSerialInitializedByPg",
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
    //             "pg_type": "BoolAsBool",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
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
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
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
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "not_null_or_nullable": "Nullable",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "NotNull"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "not_null_or_nullable": "NotNull",
    //             "pg_type_pattern": {
    //                 "ArrayDimension1": {
    //                     "dimension1_not_null_or_nullable": "Nullable"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
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
