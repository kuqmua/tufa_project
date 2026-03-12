//todo
pub use pg_types_cmn::{PgnStartsWithOne, PgnStartsWithOneTryNewEr};
gen_pg_types::gen_pg_types!({
    "pg_table_columns_write_into_file": "False",
    "whole_write_into_file": "False",
    "vrt":
    "All"
    // {
    //     "Concrete": [
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsInt2",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I32AsInt4",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I64AsInt8",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F32AsFloat4",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "F64AsFloat8",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "I16AsSmallSerialInitByPg",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "I32AsSerialInitByPg",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "I64AsBigSerialInitByPg",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgMoneyAsMoney",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "BoolAsBool",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StringAsText",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "StdVecVecU8AsBytea",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveTimeAsTime",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesTimeTimeAsTime",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgIntervalAsInterval",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateAsDate",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidV4InitByPg",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesUuidUuidAsUuidInitByClient",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesIpnetworkIpNetworkAsInet",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxTypesMacAddressMacAddressAsMacAddr",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI32AsInt4Range",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeI64AsInt8Range",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nl": "False",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nl": "True",
    //             "pg_type_pattern": "Stdrt"
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "False"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nl": "False",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         },
    //         {
    //             "pg_type": "SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange",
    //             "is_nl": "True",
    //             "pg_type_pattern": {
    //                 "ArrDim1": {
    //                     "dim1_is_nl": "True"
    //                 }
    //             }
    //         }
    //     ]
    // }
});
