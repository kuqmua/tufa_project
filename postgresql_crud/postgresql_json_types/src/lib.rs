// // generate_postgresql_json_types::generate_postgresql_json_types!("All");
// generate_postgresql_json_types::generate_postgresql_json_types!({
//     "Concrete": [
//         {
//             "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         }
//         ,
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": "Standart"
//         // }
//         // ,
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": "Standart"
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension1": {
// //                     "dimension1_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension2": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension3": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "NotNull",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "NotNull",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "NotNull",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "NotNull",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "NotNull"
// //                 }
// //             }
// //         },
// //         {
// //             "postgresql_json_type": "StdStringStringAsJsonbString",
// //             "not_null_or_nullable": "Nullable",
// //             "postgresql_json_type_pattern": {
// //                 "ArrayDimension4": {
// //                     "dimension1_not_null_or_nullable": "Nullable",
// //                     "dimension2_not_null_or_nullable": "Nullable",
// //                     "dimension3_not_null_or_nullable": "Nullable",
// //                     "dimension4_not_null_or_nullable": "Nullable"
// //                 }
// //             }
// //         },
//         {
//             "postgresql_json_type": "UuidUuidAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         }
//         // ,
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": "Standart"
//         // }
//         // ,
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // }
//         // ,
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension3": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "NotNull",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "NotNull",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension4": {
//         //             "dimension1_not_null_or_nullable": "Nullable",
//         //             "dimension2_not_null_or_nullable": "Nullable",
//         //             "dimension3_not_null_or_nullable": "Nullable",
//         //             "dimension4_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // }
//     ]
// });

#[derive(Debug)]
pub struct StdPrimitiveI8AsNotNullJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberOrigin(std::primitive::i8);
impl StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    pub fn new(value: std::primitive::i8) -> Self {
        Self(value)
    }
}
impl std::convert::From<StdPrimitiveI8AsNotNullJsonbNumberCreate> for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn from(value: StdPrimitiveI8AsNotNullJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<std::primitive::i8> for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn into(self) -> std::primitive::i8 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI8AsNotNullJsonbNumberUpdate> for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn from(value: StdPrimitiveI8AsNotNullJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl schemars::JsonSchema for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn schema_name() -> std::string::String {
        "StdPrimitiveI8AsNotNullJsonbNumberOrigin".to_owned()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("postgresql_crud::postgersql_json_type::StdPrimitiveI8AsNotNullJsonbNumberOrigin")
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            metadata: Some(Box::new(schemars::schema::Metadata {
                id: None,
                title: Some("StdPrimitiveI8AsNotNullJsonbNumberOrigin".to_owned()),
                description: None,
                default: None,
                deprecated: false,
                read_only: false,
                write_only: false,
                examples: std::vec::Vec::default(),
            })),
            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::Number))),
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: None,
            number: Some(Box::new(schemars::schema::NumberValidation {
                multiple_of: None,
                maximum: Some(std::primitive::i8::MAX as std::primitive::f64),
                exclusive_maximum: None,
                minimum: Some(std::primitive::i8::MIN as std::primitive::f64),
                exclusive_minimum: None,
            })),
            string: None,
            array: None,
            object: None,
            reference: None,
            extensions: schemars::Map::default(),
        })
    }
}
impl std::fmt::Display for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<std::primitive::i8> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<std::primitive::i8> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration {
    pub fn new(value: std::primitive::i8) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberCreate(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberCreate {
    pub fn new(value: std::primitive::i8) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8AsNotNullJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl std::convert::Into<std::primitive::i8> for StdPrimitiveI8AsNotNullJsonbNumberCreate {
    fn into(self) -> std::primitive::i8 {
        self.0.into()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery {
    pub fn new(value: std::primitive::i8) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
    }
}
impl std::convert::Into<StdPrimitiveI8AsNotNullJsonbNumberOrigin> for StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery {
    fn into(self) -> StdPrimitiveI8AsNotNullJsonbNumberOrigin {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI8AsNotNullJsonbNumberCreate> for StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery {
    fn from(value: StdPrimitiveI8AsNotNullJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
impl std::convert::From<StdPrimitiveI8AsNotNullJsonbNumberUpdate> for StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery {
    fn from(value: StdPrimitiveI8AsNotNullJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <StdPrimitiveI8AsNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <StdPrimitiveI8AsNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberSelect;
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8AsNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {}
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for StdPrimitiveI8AsNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum StdPrimitiveI8AsNotNullJsonbNumberWhereElement {
    Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual<StdPrimitiveI8AsNotNullJsonbNumberOrigin>),
    In(where_element_filters::PostgresqlJsonTypeWhereElementIn<StdPrimitiveI8AsNotNullJsonbNumberOrigin>),
    GreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberOrigin>),
    Between(where_element_filters::PostgresqlJsonTypeWhereElementBetween<StdPrimitiveI8AsNotNullJsonbNumberOrigin>),
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for StdPrimitiveI8AsNotNullJsonbNumberWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::In(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::In(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI8AsNotNullJsonbNumberWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8AsNotNullJsonbNumberWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::In(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberRead(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberRead {
    pub fn new(value: std::primitive::i8) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8AsNotNullJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type StdPrimitiveI8AsNotNullJsonbNumberReadInner = std::primitive::i8;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberUpdate(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberUpdate {
    pub fn new(value: std::primitive::i8) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
    }
}
impl std::fmt::Display for StdPrimitiveI8AsNotNullJsonbNumberUpdate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8AsNotNullJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberUpdateForQuery(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberUpdateForQuery {
    pub fn new(value: std::primitive::i8) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<StdPrimitiveI8AsNotNullJsonbNumberUpdate> for StdPrimitiveI8AsNotNullJsonbNumberUpdateForQuery {
    fn from(value: StdPrimitiveI8AsNotNullJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <StdPrimitiveI8AsNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <StdPrimitiveI8AsNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for StdPrimitiveI8AsNotNullJsonbNumber {
    type TableTypeDeclaration = StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration;
    type Create = StdPrimitiveI8AsNotNullJsonbNumberCreate;
    type CreateForQuery = StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery;
    type Select = StdPrimitiveI8AsNotNullJsonbNumberSelect;
    fn select_query_part(_: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',({column_name_and_maybe_field_getter}->'{field_ident}')))")
    }
    type WhereElement = StdPrimitiveI8AsNotNullJsonbNumberWhereElement;
    type Read = StdPrimitiveI8AsNotNullJsonbNumberRead;
    type ReadOnlyIds = StdPrimitiveI8AsNotNullJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = StdPrimitiveI8AsNotNullJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = StdPrimitiveI8AsNotNullJsonbNumberUpdate;
    type UpdateForQuery = StdPrimitiveI8AsNotNullJsonbNumberOrigin;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        Ok(format!("'{field_ident}',jsonb_build_object('value','null'::jsonb),"))
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        Ok(format!("'{field_ident}',jsonb_build_object('value','null'::jsonb),"))
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for StdPrimitiveI8AsNotNullJsonbNumber {
    type Element = Self;
    fn read_inner_vec_vec(read_only_ids: &<Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        vec![postgresql_crud_common::std_primitive_i8_test_cases_vec().into()]
    }
    fn read_new_or_try_new_unwraped_for_test(value: std::primitive::i8) -> <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn update_new_or_try_new_unwraped_for_test(value: std::primitive::i8) -> <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(<<StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        StdPrimitiveI8AsNotNullJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn read_from_previous_read_unwraped_merged_with_update(read: <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => StdPrimitiveI8AsNotNullJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn create_vec() -> std::vec::Vec<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::Create> {
        postgresql_crud_common::std_primitive_i8_test_cases_vec().into_iter().map(|element| <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(element)).collect()
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value { value: StdPrimitiveI8AsNotNullJsonbNumberRead(create.into()) })
    }
}
#[derive(Debug)]
pub struct UuidUuidAsNotNullJsonbString;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UuidUuidAsNotNullJsonbStringOrigin(uuid::Uuid);
impl UuidUuidAsNotNullJsonbStringOrigin {
    pub fn new(value: uuid::Uuid) -> Self {
        Self(value)
    }
}
impl postgresql_crud_common::PostgresqlJsonTypeElementId for UuidUuidAsNotNullJsonbString {
    type CreateForQuery = UuidUuidAsNotNullJsonbStringCreateForQuery;
    type UpdateForQuery = UuidUuidAsNotNullJsonbStringUpdateForQuery;
    type Origin = UuidUuidAsNotNullJsonbStringOrigin;
    type Inner = uuid::Uuid;
    fn query_bind_string_as_postgresql_text_create_for_query(value: Self::CreateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value.0.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn query_bind_string_as_postgresql_text_update_for_query(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value.0.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn query_bind_string_as_postgresql_text_origin(value: Self::Origin, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value.0.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn get_inner<'a>(value: &'a Self::CreateForQuery) -> &'a Self::Inner {
        &value.0.0
    }
}
impl std::convert::From<UuidUuidAsNotNullJsonbStringCreate> for UuidUuidAsNotNullJsonbStringOrigin {
    fn from(value: UuidUuidAsNotNullJsonbStringCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<uuid::Uuid> for UuidUuidAsNotNullJsonbStringOrigin {
    fn into(self) -> uuid::Uuid {
        self.0
    }
}
impl std::convert::From<UuidUuidAsNotNullJsonbStringUpdate> for UuidUuidAsNotNullJsonbStringOrigin {
    fn from(value: UuidUuidAsNotNullJsonbStringUpdate) -> Self {
        value.0
    }
}
impl schemars::JsonSchema for UuidUuidAsNotNullJsonbStringOrigin {
    fn schema_name() -> std::string::String {
        "UuidUuidAsNotNullJsonbStringOrigin".to_owned()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("postgresql_crud::postgersql_json_type::UuidUuidAsNotNullJsonbStringOrigin")
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            metadata: Some(Box::new(schemars::schema::Metadata {
                id: None,
                title: Some("UuidUuidAsNotNullJsonbStringOrigin".to_owned()),
                description: None,
                default: None,
                deprecated: false,
                read_only: false,
                write_only: false,
                examples: std::vec::Vec::default(),
            })),
            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::String))),
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: None,
            number: None,
            string: Some(Box::new(schemars::schema::StringValidation { max_length: Some(36), min_length: Some(36), pattern: None })),
            array: None,
            object: None,
            reference: None,
            extensions: schemars::Map::default(),
        })
    }
}
impl postgresql_crud_common::IsStringEmpty for UuidUuidAsNotNullJsonbStringOrigin {
    fn is_string_empty(&self) -> std::primitive::bool {
        self.0.to_string().is_empty()
    }
}
impl std::fmt::Display for UuidUuidAsNotNullJsonbStringOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for UuidUuidAsNotNullJsonbStringOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
impl sqlx::Type<sqlx::Postgres> for UuidUuidAsNotNullJsonbStringOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<uuid::Uuid> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<uuid::Uuid> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UuidUuidAsNotNullJsonbStringOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringTableTypeDeclaration(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringTableTypeDeclaration {
    pub fn new(value: uuid::Uuid) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringCreate(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringCreate {
    pub fn new(value: uuid::Uuid) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl std::convert::Into<uuid::Uuid> for UuidUuidAsNotNullJsonbStringCreate {
    fn into(self) -> uuid::Uuid {
        self.0.into()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct UuidUuidAsNotNullJsonbStringCreateForQuery(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringCreateForQuery {
    pub fn new(value: uuid::Uuid) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl std::convert::Into<UuidUuidAsNotNullJsonbStringOrigin> for UuidUuidAsNotNullJsonbStringCreateForQuery {
    fn into(self) -> UuidUuidAsNotNullJsonbStringOrigin {
        self.0
    }
}
impl std::convert::From<UuidUuidAsNotNullJsonbStringCreate> for UuidUuidAsNotNullJsonbStringCreateForQuery {
    fn from(value: UuidUuidAsNotNullJsonbStringCreate) -> Self {
        Self(value.0)
    }
}
impl std::convert::From<UuidUuidAsNotNullJsonbStringUpdate> for UuidUuidAsNotNullJsonbStringCreateForQuery {
    fn from(value: UuidUuidAsNotNullJsonbStringUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UuidUuidAsNotNullJsonbStringCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for UuidUuidAsNotNullJsonbStringCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <UuidUuidAsNotNullJsonbStringOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <UuidUuidAsNotNullJsonbStringOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringSelect;
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {}
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for UuidUuidAsNotNullJsonbStringSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum UuidUuidAsNotNullJsonbStringWhereElement {
    Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual<UuidUuidAsNotNullJsonbStringOrigin>),
    In(where_element_filters::PostgresqlJsonTypeWhereElementIn<UuidUuidAsNotNullJsonbStringOrigin>),
    RegularExpression(where_element_filters::PostgresqlJsonTypeWhereElementRegularExpression),
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for UuidUuidAsNotNullJsonbStringWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::In(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::RegularExpression(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::In(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::RegularExpression(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for UuidUuidAsNotNullJsonbStringWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::In(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::RegularExpression(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringRead(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringRead {
    pub fn new(value: uuid::Uuid) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UuidUuidAsNotNullJsonbStringReadOnlyIds(pub postgresql_crud_common::Value<uuid::Uuid>);
pub type UuidUuidAsNotNullJsonbStringReadInner = uuid::Uuid;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringUpdate(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringUpdate {
    pub fn new(value: uuid::Uuid) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl std::fmt::Display for UuidUuidAsNotNullJsonbStringUpdate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct UuidUuidAsNotNullJsonbStringUpdateForQuery(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringUpdateForQuery {
    pub fn new(value: uuid::Uuid) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl std::convert::From<UuidUuidAsNotNullJsonbStringUpdate> for UuidUuidAsNotNullJsonbStringUpdateForQuery {
    fn from(value: UuidUuidAsNotNullJsonbStringUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UuidUuidAsNotNullJsonbStringUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for UuidUuidAsNotNullJsonbStringUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <UuidUuidAsNotNullJsonbStringOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <UuidUuidAsNotNullJsonbStringOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for UuidUuidAsNotNullJsonbString {
    type TableTypeDeclaration = UuidUuidAsNotNullJsonbStringTableTypeDeclaration;
    type Create = UuidUuidAsNotNullJsonbStringCreate;
    type CreateForQuery = UuidUuidAsNotNullJsonbStringCreateForQuery;
    type Select = UuidUuidAsNotNullJsonbStringSelect;
    fn select_query_part(_: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',({column_name_and_maybe_field_getter}->'{field_ident}')))")
    }
    type WhereElement = UuidUuidAsNotNullJsonbStringWhereElement;
    type Read = UuidUuidAsNotNullJsonbStringRead;
    type ReadOnlyIds = UuidUuidAsNotNullJsonbStringReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!("jsonb_build_object('value',{column_name_and_maybe_field_getter})")
    }
    type ReadInner = UuidUuidAsNotNullJsonbStringReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = UuidUuidAsNotNullJsonbStringUpdate;
    type UpdateForQuery = UuidUuidAsNotNullJsonbStringOrigin;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("'{field_ident}',jsonb_build_object('value',${increment}),"))
            }
            None => Err(postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("'{field_ident}',jsonb_build_object('value',${increment}),"))
            }
            None => Err(postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for UuidUuidAsNotNullJsonbString {
    type Element = Self;
    fn read_inner_vec_vec(read_only_ids: &<Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        vec![]
    }
    fn read_new_or_try_new_unwraped_for_test(value: uuid::Uuid) -> <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn update_new_or_try_new_unwraped_for_test(value: uuid::Uuid) -> <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value { value: value.0.value })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        UuidUuidAsNotNullJsonbStringReadOnlyIds(postgresql_crud_common::Value { value: value.0.clone().into() })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value { value: UuidUuidAsNotNullJsonbStringRead::new(value.0.value) })
    }
    fn read_from_previous_read_unwraped_merged_with_update(read: <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => UuidUuidAsNotNullJsonbStringRead(value.into()),
            None => read,
        }
    }
    fn create_vec() -> std::vec::Vec<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::Create> {
        vec![]
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::Element as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::Element as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::Element as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: UuidUuidAsNotNullJsonbStringRead(UuidUuidAsNotNullJsonbStringOrigin::new(read_only_ids.0.value)),
        })
    }
}
