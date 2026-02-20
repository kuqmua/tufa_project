//todo
pub use pg_types_common::{PaginationStartsWithOne, PaginationStartsWithOneTryNewEr};
gen_pg_types::gen_pg_types!({
    "pg_table_columns_write_into_file": "False",
    "whole_write_into_file": "False",
    "vrt":
    "All"
    // {
    //     "Concrete": [
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsSmallSerialInitByPg",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I32AsSerialInitByPg",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "I64AsBigSerialInitByPg",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidV4InitByPg",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nullable": "False",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nullable": "True",
    //             "pg_type_pattern": "Standart"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nullable": "False",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nullable": "True",
    //             "pg_type_pattern": {
    //                 "ArrayDim1": {
    //                     "dim1_is_nullable": "True"
    //                 }
    //             }
    //         }
    //     ]
    // }
});
