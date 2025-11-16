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
//         {
//             "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
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
//         // {
//         //     "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
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
//         // }
//         // ,
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // }
//         // ,
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // }
//         // ,
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
//         // }
//         // ,
//         // {
//         //     "postgresql_json_type": "UuidUuidAsJsonbString",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_json_type_pattern": {
//         //         "ArrayDimension2": {
//         //             "dimension1_not_null_or_nullable": "NotNull",
//         //             "dimension2_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // }
//         // ,
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
//         // }
//         // ,
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

fn ok_field_ident_jsonb_build_object_value(field_ident: &std::primitive::str) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
    Ok(format!("'{field_ident}',jsonb_build_object('value','null'::jsonb),"))
}

// //
#[derive(Debug)]
pub struct StdPrimitiveI8AsNotNullJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
struct StdPrimitiveI8AsNotNullJsonbNumberOrigin(std::primitive::i8);
impl StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    pub fn new(value: StdPrimitiveI8AsNotNullJsonbNumberReadInner) -> Self {
        Self(value)
    }
}
impl std::convert::From<StdPrimitiveI8AsNotNullJsonbNumberCreate> for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn from(value: StdPrimitiveI8AsNotNullJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<StdPrimitiveI8AsNotNullJsonbNumberReadInner> for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn into(self) -> StdPrimitiveI8AsNotNullJsonbNumberReadInner {
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
        <sqlx::types::Json<StdPrimitiveI8AsNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<StdPrimitiveI8AsNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration {
    pub fn new(value: StdPrimitiveI8AsNotNullJsonbNumberReadInner) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<StdPrimitiveI8AsNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<StdPrimitiveI8AsNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberCreate(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberCreate {
    pub fn new(value: StdPrimitiveI8AsNotNullJsonbNumberReadInner) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8AsNotNullJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery {
    pub fn new(value: StdPrimitiveI8AsNotNullJsonbNumberReadInner) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
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
impl std::convert::From<StdPrimitiveI8AsNotNullJsonbNumberCreate> for StdPrimitiveI8AsNotNullJsonbNumberCreateForQuery {
    fn from(value: StdPrimitiveI8AsNotNullJsonbNumberCreate) -> Self {
        Self(value.0)
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
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum StdPrimitiveI8AsNotNullJsonbNumberWhereElement {
    Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    In(where_element_filters::PostgresqlJsonTypeWhereElementIn<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    GreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    Between(where_element_filters::PostgresqlJsonTypeWhereElementBetween<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
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
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberRead(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberRead {
    pub fn new(value: StdPrimitiveI8AsNotNullJsonbNumberReadInner) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI8AsNotNullJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI8AsNotNullJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<StdPrimitiveI8AsNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<StdPrimitiveI8AsNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type StdPrimitiveI8AsNotNullJsonbNumberReadInner = std::primitive::i8;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberUpdate(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberUpdate {
    pub fn new(value: StdPrimitiveI8AsNotNullJsonbNumberReadInner) -> Self {
        Self(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value))
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
    pub fn new(value: StdPrimitiveI8AsNotNullJsonbNumberReadInner) -> Self {
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
        value.0 .0
    }
    type Update = StdPrimitiveI8AsNotNullJsonbNumberUpdate;
    type UpdateForQuery = StdPrimitiveI8AsNotNullJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for StdPrimitiveI8AsNotNullJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = StdPrimitiveI8AsNotNullJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        postgresql_crud_common::std_primitive_i8_test_cases_vec().into_iter().map(|element| <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(element)).collect()
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        vec![postgresql_crud_common::std_primitive_i8_test_cases_vec().into()]
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: StdPrimitiveI8AsNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: StdPrimitiveI8AsNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(<<StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        StdPrimitiveI8AsNotNullJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => StdPrimitiveI8AsNotNullJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        StdPrimitiveI8AsNotNullJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        StdPrimitiveI8AsNotNullJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
            logical_operator: postgresql_crud_common::LogicalOperator::Or,
            value: StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration::new(create.0.into()),
        })
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        None
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        None
    }
}
#[derive(Debug)]
pub struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin(std::vec::Vec<StdPrimitiveI8AsNotNullJsonbNumberOrigin>);
impl VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin {
    pub fn new(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(value.into_iter().map(|element| StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(element)).collect())
    }
}
impl std::convert::From<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreate> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn from(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn into(self) -> VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner {
        self.0.into_iter().map(|element| element.0).collect()
    }
}
impl std::convert::From<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdate> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn from(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl std::fmt::Display for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    pub fn new(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreate(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreate {
    pub fn new(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreateForQuery(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    pub fn new(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl std::convert::From<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreate> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    fn from(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberSelect {
    dimension1_pagination: postgresql_crud_common::PaginationStartsWithZero,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberWhereElement {
    Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneLengthEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneContainsAllElementsOfArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneOverlapsWithArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneOverlapsWithArray<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    AllElementsEqual(where_element_filters::PostgresqlJsonTypeWhereElementAllElementsEqual<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneIn(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneIn<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneBetween(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneBetween<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    ContainsElementGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementContainsElementGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    AllElementsGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementAllElementsGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::ContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::ContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::AllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneContainsAllElementsOfArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneOverlapsWithArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneBetween(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ContainsElementGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn new(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner = std::vec::Vec<std::primitive::i8>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdate(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdate {
    pub fn new(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdateForQuery(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    pub fn new(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdate> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    fn from(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber {
    type TableTypeDeclaration = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration;
    type Create = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreate;
    type CreateForQuery = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreateForQuery;
    type Select = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg((value)) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {dimension1_start} and {dimension1_end})))")
    }
    type WhereElement = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberWhereElement;
    type Read = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberRead;
    type ReadOnlyIds = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0 .0.into_iter().map(|element| element.0).collect()
    }
    type Update = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdate;
    type UpdateForQuery = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        let mut acc = vec![];
        for element in <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
            acc.push(<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(vec![element.0.into()]));
        }
        acc.push(<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new({
            let mut acc = vec![];
            for element in <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
                acc.push(element.0.into());
            }
            acc
        }));
        acc
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        let read_only_ids_to_two_dimensional_vec_read_inner = <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&StdPrimitiveI8AsNotNullJsonbNumberReadOnlyIds(read_only_ids.0.clone()));
        let option_additional = {
            let mut option_additional = None;
            for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                if option_additional.is_some() {
                    break;
                }
                for element1 in element0 {
                    if option_additional.is_none() {
                        option_additional = Some((vec![vec![element1.clone()]], vec![vec![element1.clone(), element1.clone()]]));
                    } else {
                        break;
                    }
                }
            }
            option_additional
        };
        let has_len_more_than_one = {
            let mut has_len_more_than_one = false;
            for element in &read_only_ids_to_two_dimensional_vec_read_inner {
                if element.len() > 1 {
                    has_len_more_than_one = true;
                    break;
                }
            }
            has_len_more_than_one
        };
        acc.push(vec![{
            let mut acc = vec![];
            for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                for element1 in element0 {
                    acc.push(element1);
                }
            }
            acc
        }]);
        if let Some(value) = option_additional {
            if has_len_more_than_one {
                acc.push(value.0);
            }
            if !has_len_more_than_one {
                acc.push(value.1);
            }
        }
        acc
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(
                <<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            ),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
            logical_operator: postgresql_crud_common::LogicalOperator::Or,
            value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration::new(create.0.into()),
        })
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0 .0.into_iter().enumerate() {
                acc.push(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberWhereElement::DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual {
                    logical_operator: postgresql_crud_common::LogicalOperator::And,
                    dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                    value: StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration(element),
                }));
            }
            acc
        })
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        None
    }
}
#[derive(Debug)]
pub struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin(std::vec::Vec<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin>);
impl VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    pub fn new(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(value.into_iter().map(|element| VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin::new(element)).collect())
    }
}
impl std::convert::From<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreate> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn from(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn into(self) -> VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner {
        self.0.into_iter().map(|element| element.0.into_iter().map(|element| element.0).collect()).collect()
    }
}
impl std::convert::From<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdate> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn from(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl std::fmt::Display for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    pub fn new(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreate(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreate {
    pub fn new(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    pub fn new(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl std::convert::From<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreate> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    fn from(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberSelect {
    dimension1_pagination: postgresql_crud_common::PaginationStartsWithZero,
    dimension2_pagination: postgresql_crud_common::PaginationStartsWithZero,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimension2_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
            dimension2_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement {
    Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration>),
    DimensionTwoEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoEqual<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneLengthEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthEqual),
    DimensionTwoLengthEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoLengthEqual),
    DimensionOneLengthMoreThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthMoreThan),
    DimensionTwoLengthMoreThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoLengthMoreThan),
    DimensionTwoContainsAllElementsOfArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoContainsAllElementsOfArray<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionTwoOverlapsWithArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoOverlapsWithArray<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    AllElementsEqual(where_element_filters::PostgresqlJsonTypeWhereElementAllElementsEqual<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneAllElementsEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneAllElementsEqual<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneIn(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneIn<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration>),
    DimensionTwoIn(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoIn<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionTwoGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionTwoBetween(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoBetween<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneContainsElementGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneContainsElementGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneAllElementsGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneAllElementsGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneAllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneAllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoContainsAllElementsOfArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoOverlapsWithArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAllElementsEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoBetween(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneContainsElementGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAllElementsGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn new(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner = std::vec::Vec<std::vec::Vec<std::primitive::i8>>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdate(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdate {
    pub fn new(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    pub fn new(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdate> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    fn from(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber {
    type TableTypeDeclaration = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration;
    type Create = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreate;
    type CreateForQuery = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery;
    type Select = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        let dimension2_start = value.dimension2_pagination.start();
        let dimension2_end = value.dimension2_pagination.end();
        format ! ("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg((select jsonb_agg((d2_elem.value)) from jsonb_array_elements((d1_elem.value)) with ordinality as d2_elem(value, d2_elem) where d2_elem between {dimension2_start} and {dimension2_end})) from jsonb_array_elements(({column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality as d1_elem(value, d1_elem) where d1_elem between {dimension1_start} and {dimension1_end})))")
    }
    type WhereElement = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement;
    type Read = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead;
    type ReadOnlyIds = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0 .0.into_iter().map(|element| element.0.into_iter().map(|element| element.0).collect()).collect()
    }
    type Update = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdate;
    type UpdateForQuery = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        let mut acc = vec![];
        for element in <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
            acc.push(<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(vec![element.0.into()]));
        }
        acc.push(<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new({
            let mut acc = vec![];
            for element in <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
                acc.push(element.0.into());
            }
            acc
        }));
        acc
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        let read_only_ids_to_two_dimensional_vec_read_inner = <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadOnlyIds(read_only_ids.0.clone()));
        let option_additional = {
            let mut option_additional = None;
            for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                if option_additional.is_some() {
                    break;
                }
                for element1 in element0 {
                    if option_additional.is_none() {
                        option_additional = Some((vec![vec![element1.clone()]], vec![vec![element1.clone(), element1.clone()]]));
                    } else {
                        break;
                    }
                }
            }
            option_additional
        };
        let has_len_more_than_one = read_only_ids_to_two_dimensional_vec_read_inner.len() > 1;
        acc.push(vec![{
            let mut acc = vec![];
            for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                for element1 in element0 {
                    acc.push(element1);
                }
            }
            acc
        }]);
        if let Some(value) = option_additional {
            if has_len_more_than_one {
                acc.push(value.0);
            }
            if !has_len_more_than_one {
                acc.push(value.1);
            }
        }
        acc
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(
                <<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            ),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
            logical_operator: postgresql_crud_common::LogicalOperator::Or,
            value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration::new(create.0.into()),
        })
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0 .0.into_iter().enumerate() {
                acc.push(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement::DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual {
                    logical_operator: postgresql_crud_common::LogicalOperator::And,
                    dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                    value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration(element),
                }));
            }
            acc
        })
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0 .0.into_iter().enumerate() {
                for element in element.0 {
                    acc.push(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement::DimensionTwoEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoEqual {
                        logical_operator: postgresql_crud_common::LogicalOperator::And,
                        dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                        value: StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration::new(element.into()),
                    }));
                }
            }
            acc
        })
    }
}
#[derive(Debug)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
struct OptionStdPrimitiveI8AsNullableJsonbNumberOrigin(std::option::Option<StdPrimitiveI8AsNotNullJsonbNumberOrigin>);
impl OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    pub fn new(value: OptionStdPrimitiveI8AsNullableJsonbNumberReadInner) -> Self {
        Self(match value {
            Some(value) => Some(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value)),
            None => None,
        })
    }
}
impl std::convert::From<OptionStdPrimitiveI8AsNullableJsonbNumberCreate> for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn from(value: OptionStdPrimitiveI8AsNullableJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<OptionStdPrimitiveI8AsNullableJsonbNumberReadInner> for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn into(self) -> OptionStdPrimitiveI8AsNullableJsonbNumberReadInner {
        match self.0 {
            Some(value) => Some(value.0),
            None => None,
        }
    }
}
impl std::convert::From<OptionStdPrimitiveI8AsNullableJsonbNumberUpdate> for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn from(value: OptionStdPrimitiveI8AsNullableJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl std::fmt::Display for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionStdPrimitiveI8AsNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionStdPrimitiveI8AsNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration {
    pub fn new(value: OptionStdPrimitiveI8AsNullableJsonbNumberReadInner) -> Self {
        Self(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionStdPrimitiveI8AsNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionStdPrimitiveI8AsNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberCreate(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveI8AsNullableJsonbNumberCreate {
    pub fn new(value: OptionStdPrimitiveI8AsNullableJsonbNumberReadInner) -> Self {
        Self(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionStdPrimitiveI8AsNullableJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberCreateForQuery(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveI8AsNullableJsonbNumberCreateForQuery {
    pub fn new(value: OptionStdPrimitiveI8AsNullableJsonbNumberReadInner) -> Self {
        Self(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin::new(value))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionStdPrimitiveI8AsNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <OptionStdPrimitiveI8AsNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl std::convert::From<OptionStdPrimitiveI8AsNullableJsonbNumberCreate> for OptionStdPrimitiveI8AsNullableJsonbNumberCreateForQuery {
    fn from(value: OptionStdPrimitiveI8AsNullableJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberSelect;
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionStdPrimitiveI8AsNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {}
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for OptionStdPrimitiveI8AsNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {}
    }
}
pub type OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement = postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter<<StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::WhereElement>;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberRead(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveI8AsNullableJsonbNumberRead {
    pub fn new(value: OptionStdPrimitiveI8AsNullableJsonbNumberReadInner) -> Self {
        Self(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionStdPrimitiveI8AsNullableJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionStdPrimitiveI8AsNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionStdPrimitiveI8AsNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type OptionStdPrimitiveI8AsNullableJsonbNumberReadInner = std::option::Option<std::primitive::i8>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberUpdate(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveI8AsNullableJsonbNumberUpdate {
    pub fn new(value: OptionStdPrimitiveI8AsNullableJsonbNumberReadInner) -> Self {
        Self(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionStdPrimitiveI8AsNullableJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberUpdateForQuery(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveI8AsNullableJsonbNumberUpdateForQuery {
    pub fn new(value: OptionStdPrimitiveI8AsNullableJsonbNumberReadInner) -> Self {
        Self(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<OptionStdPrimitiveI8AsNullableJsonbNumberUpdate> for OptionStdPrimitiveI8AsNullableJsonbNumberUpdateForQuery {
    fn from(value: OptionStdPrimitiveI8AsNullableJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionStdPrimitiveI8AsNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <OptionStdPrimitiveI8AsNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for OptionStdPrimitiveI8AsNullableJsonbNumber {
    type TableTypeDeclaration = OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration;
    type Create = OptionStdPrimitiveI8AsNullableJsonbNumberCreate;
    type CreateForQuery = OptionStdPrimitiveI8AsNullableJsonbNumberCreateForQuery;
    type Select = OptionStdPrimitiveI8AsNullableJsonbNumberSelect;
    fn select_query_part(_: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}')='null' then 'null'::jsonb else ({column_name_and_maybe_field_getter}->'{field_ident}') end)))")
    }
    type WhereElement = OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement;
    type Read = OptionStdPrimitiveI8AsNullableJsonbNumberRead;
    type ReadOnlyIds = OptionStdPrimitiveI8AsNullableJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = OptionStdPrimitiveI8AsNullableJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(value.0),
            None => None,
        }
    }
    type Update = OptionStdPrimitiveI8AsNullableJsonbNumberUpdate;
    type UpdateForQuery = OptionStdPrimitiveI8AsNullableJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for OptionStdPrimitiveI8AsNullableJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = OptionStdPrimitiveI8AsNullableJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        let mut acc = vec![];
        for element in <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
            acc.push(<OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(Some(element.0.into())));
        }
        acc.push(<OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(None));
        acc
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        for element0 in <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&StdPrimitiveI8AsNotNullJsonbNumberReadOnlyIds(read_only_ids.0.clone())) {
            for element1 in element0 {
                acc.push(vec![Some(element1)]);
            }
        }
        acc.push(vec![None]);
        acc
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: OptionStdPrimitiveI8AsNullableJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: OptionStdPrimitiveI8AsNullableJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(<<OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        OptionStdPrimitiveI8AsNullableJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => OptionStdPrimitiveI8AsNullableJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        OptionStdPrimitiveI8AsNullableJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        match create.0 .0 {
            Some(value) => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(Some(
                postgresql_crud_common::NotEmptyUniqueEnumVec::try_new(vec![StdPrimitiveI8AsNotNullJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
                    logical_operator: postgresql_crud_common::LogicalOperator::Or,
                    value: StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration::new(value.into()),
                })])
                .expect("error 88bfa095-a3ab-4d0c-be71-af63c3acd50f"),
            )),
            None => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(None),
        }
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        None
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        None
    }
}
#[derive(Debug)]
pub struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin(std::vec::Vec<OptionStdPrimitiveI8AsNullableJsonbNumberOrigin>);
impl VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin {
    pub fn new(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(value.into_iter().map(|element| OptionStdPrimitiveI8AsNullableJsonbNumberOrigin::new(element)).collect())
    }
}
impl std::convert::From<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreate> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin {
    fn from(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin {
    fn into(self) -> VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner {
        self.0
            .into_iter()
            .map(|element| match element.0 {
                Some(value) => Some(value.0),
                None => None,
            })
            .collect()
    }
}
impl std::convert::From<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdate> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin {
    fn from(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl std::fmt::Display for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration {
    pub fn new(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreate(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreate {
    pub fn new(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreateForQuery(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreateForQuery {
    pub fn new(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl std::convert::From<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreate> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreateForQuery {
    fn from(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberSelect {
    dimension1_pagination: postgresql_crud_common::PaginationStartsWithZero,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberWhereElement {
    Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneLengthEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneContainsAllElementsOfArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneOverlapsWithArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneOverlapsWithArray<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    AllElementsEqual(where_element_filters::PostgresqlJsonTypeWhereElementAllElementsEqual<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneIn(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneIn<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneGreaterThan<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneBetween(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneBetween<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    ContainsElementGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementContainsElementGreaterThan<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    AllElementsGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementAllElementsGreaterThan<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::ContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::ContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::AllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneContainsAllElementsOfArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneOverlapsWithArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneBetween(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ContainsElementGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn new(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner = std::vec::Vec<std::option::Option<std::primitive::i8>>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdate(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdate {
    pub fn new(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdateForQuery(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdateForQuery {
    pub fn new(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdate> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdateForQuery {
    fn from(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber {
    type TableTypeDeclaration = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration;
    type Create = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreate;
    type CreateForQuery = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreateForQuery;
    type Select = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg((value)) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {dimension1_start} and {dimension1_end})))")
    }
    type WhereElement = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberWhereElement;
    type Read = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberRead;
    type ReadOnlyIds = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value
            .0
             .0
            .into_iter()
            .map(|element| match element.0 {
                Some(value) => Some(value.0),
                None => None,
            })
            .collect()
    }
    type Update = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdate;
    type UpdateForQuery = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        let mut acc = vec![];
        for element in <OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
            acc.push(<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(vec![element.0.into()]));
        }
        acc.push(<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new({
            let mut acc = vec![];
            for element in <OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
                acc.push(element.0.into());
            }
            acc
        }));
        acc
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        let read_only_ids_to_two_dimensional_vec_read_inner = <OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&OptionStdPrimitiveI8AsNullableJsonbNumberReadOnlyIds(read_only_ids.0.clone()));
        let option_additional = {
            let mut option_additional = None;
            for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                if option_additional.is_some() {
                    break;
                }
                for element1 in element0 {
                    if option_additional.is_none() {
                        option_additional = Some((vec![vec![element1.clone()]], vec![vec![element1.clone(), element1.clone()]]));
                    } else {
                        break;
                    }
                }
            }
            option_additional
        };
        let has_len_more_than_one = read_only_ids_to_two_dimensional_vec_read_inner.len() > 1;
        acc.push(vec![{
            let mut acc = vec![];
            for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                for element1 in element0 {
                    acc.push(element1);
                }
            }
            acc
        }]);
        if let Some(value) = option_additional {
            if has_len_more_than_one {
                acc.push(value.0);
            }
            if !has_len_more_than_one {
                acc.push(value.1);
            }
        }
        acc
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(
                <<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            ),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
            logical_operator: postgresql_crud_common::LogicalOperator::Or,
            value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration::new(create.0.into()),
        })
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0 .0.into_iter().enumerate() {
                acc.push(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberWhereElement::DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual {
                    logical_operator: postgresql_crud_common::LogicalOperator::And,
                    dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                    value: OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration(element),
                }));
            }
            acc
        })
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        None
    }
}
#[derive(Debug)]
pub struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin(std::vec::Vec<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin>);
impl VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin {
    pub fn new(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(value.into_iter().map(|element| VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin::new(element)).collect())
    }
}
impl std::convert::From<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreate> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin {
    fn from(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin {
    fn into(self) -> VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner {
        self.0
            .into_iter()
            .map(|element| {
                element
                    .0
                    .into_iter()
                    .map(|element| match element.0 {
                        Some(value) => Some(value.0),
                        None => None,
                    })
                    .collect()
            })
            .collect()
    }
}
impl std::convert::From<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdate> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin {
    fn from(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl std::fmt::Display for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberTableTypeDeclaration(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberTableTypeDeclaration {
    pub fn new(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreate(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreate {
    pub fn new(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreateForQuery(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreateForQuery {
    pub fn new(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl std::convert::From<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreate> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreateForQuery {
    fn from(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberSelect {
    dimension1_pagination: postgresql_crud_common::PaginationStartsWithZero,
    dimension2_pagination: postgresql_crud_common::PaginationStartsWithZero,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimension2_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
            dimension2_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberWhereElement {
    Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration>),
    DimensionTwoEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoEqual<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneLengthEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthEqual),
    DimensionTwoLengthEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoLengthEqual),
    DimensionOneLengthMoreThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthMoreThan),
    DimensionTwoLengthMoreThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoLengthMoreThan),
    DimensionTwoContainsAllElementsOfArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoContainsAllElementsOfArray<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionTwoOverlapsWithArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoOverlapsWithArray<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    AllElementsEqual(where_element_filters::PostgresqlJsonTypeWhereElementAllElementsEqual<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneAllElementsEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneAllElementsEqual<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneIn(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneIn<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration>),
    DimensionTwoIn(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoIn<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionTwoGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoGreaterThan<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionTwoBetween(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoBetween<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneContainsElementGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneContainsElementGreaterThan<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneAllElementsGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneAllElementsGreaterThan<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneAllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneAllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoContainsAllElementsOfArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoOverlapsWithArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAllElementsEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoBetween(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneContainsElementGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAllElementsGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn new(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner = std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdate(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdate {
    pub fn new(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdateForQuery(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdateForQuery {
    pub fn new(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdate> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdateForQuery {
    fn from(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber {
    type TableTypeDeclaration = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberTableTypeDeclaration;
    type Create = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreate;
    type CreateForQuery = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberCreateForQuery;
    type Select = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        let dimension2_start = value.dimension2_pagination.start();
        let dimension2_end = value.dimension2_pagination.end();
        format ! ("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg((select jsonb_agg((d2_elem.value)) from jsonb_array_elements((d1_elem.value)) with ordinality as d2_elem(value, d2_elem) where d2_elem between {dimension2_start} and {dimension2_end})) from jsonb_array_elements(({column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality as d1_elem(value, d1_elem) where d1_elem between {dimension1_start} and {dimension1_end})))")
    }
    type WhereElement = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberWhereElement;
    type Read = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead;
    type ReadOnlyIds = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value
            .0
             .0
            .into_iter()
            .map(|element| {
                element
                    .0
                    .into_iter()
                    .map(|element| match element.0 {
                        Some(value) => Some(value.0),
                        None => None,
                    })
                    .collect()
            })
            .collect()
    }
    type Update = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdate;
    type UpdateForQuery = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        let mut acc = vec![];
        for element in <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
            acc.push(<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(vec![element.0.into()]));
        }
        acc.push(<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new({
            let mut acc = vec![];
            for element in <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
                acc.push(element.0.into());
            }
            acc
        }));
        acc
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        let read_only_ids_to_two_dimensional_vec_read_inner = <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadOnlyIds(read_only_ids.0.clone()));
        let option_additional = {
            let mut option_additional = None;
            for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                if option_additional.is_some() {
                    break;
                }
                for element1 in element0 {
                    if option_additional.is_none() {
                        option_additional = Some((vec![vec![element1.clone()]], vec![vec![element1.clone(), element1.clone()]]));
                    } else {
                        break;
                    }
                }
            }
            option_additional
        };
        let has_len_more_than_one = read_only_ids_to_two_dimensional_vec_read_inner.len() > 1;
        acc.push(vec![{
            let mut acc = vec![];
            for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                for element1 in element0 {
                    acc.push(element1);
                }
            }
            acc
        }]);
        if let Some(value) = option_additional {
            if has_len_more_than_one {
                acc.push(value.0);
            }
            if !has_len_more_than_one {
                acc.push(value.1);
            }
        }
        acc
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(
                <<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            ),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
            logical_operator: postgresql_crud_common::LogicalOperator::Or,
            value: VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberTableTypeDeclaration::new(create.0.into()),
        })
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0 .0.into_iter().enumerate() {
                acc.push(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberWhereElement::DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual {
                    logical_operator: postgresql_crud_common::LogicalOperator::And,
                    dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                    value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration(element),
                }));
            }
            acc
        })
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0 .0.into_iter().enumerate() {
                for element in element.0 {
                    acc.push(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberWhereElement::DimensionTwoEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoEqual {
                        logical_operator: postgresql_crud_common::LogicalOperator::And,
                        dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                        value: OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration::new(element.into()),
                    }));
                }
            }
            acc
        })
    }
}
#[derive(Debug)]
pub struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin(std::option::Option<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin>);
impl OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin {
    pub fn new(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(match value {
            Some(value) => Some(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin::new(value)),
            None => None,
        })
    }
}
impl std::convert::From<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreate> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin {
    fn from(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin {
    fn into(self) -> OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner {
        match self.0 {
            Some(value) => Some(value.0.into_iter().map(|element| element.0).collect()),
            None => None,
        }
    }
}
impl std::convert::From<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdate> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin {
    fn from(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl std::fmt::Display for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration {
    pub fn new(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreate(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreate {
    pub fn new(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreateForQuery(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreateForQuery {
    pub fn new(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl std::convert::From<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreate> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreateForQuery {
    fn from(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberSelect {
    dimension1_pagination: postgresql_crud_common::PaginationStartsWithZero,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
        }
    }
}
pub type OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberWhereElement = postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter<<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::WhereElement>;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn new(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner = std::option::Option<std::vec::Vec<std::primitive::i8>>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdate(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdate {
    pub fn new(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdateForQuery(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdateForQuery {
    pub fn new(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdate> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdateForQuery {
    fn from(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber {
    type TableTypeDeclaration = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration;
    type Create = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreate;
    type CreateForQuery = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberCreateForQuery;
    type Select = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}')='null' then 'null'::jsonb else (select jsonb_agg((value)) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {dimension1_start} and {dimension1_end}) end)))")
    }
    type WhereElement = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberWhereElement;
    type Read = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberRead;
    type ReadOnlyIds = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(value.0.into_iter().map(|element| element.0).collect()),
            None => None,
        }
    }
    type Update = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdate;
    type UpdateForQuery = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        let mut acc = vec![];
        for element in <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
            acc.push(<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(Some(element.0.into())));
        }
        acc.push(<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(None));
        acc
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        let read_only_ids_to_two_dimensional_vec_read_inner = <StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&StdPrimitiveI8AsNotNullJsonbNumberReadOnlyIds(read_only_ids.0.clone()));
        let option_additional = {
            let mut option_additional = None;
            for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                if option_additional.is_some() {
                    break;
                }
                for element1 in element0 {
                    if option_additional.is_none() {
                        option_additional = Some((vec![Some(vec![element1.clone()])], vec![Some(vec![element1.clone(), element1.clone()])]));
                    } else {
                        break;
                    }
                }
            }
            option_additional
        };
        let has_len_more_than_one = {
            let mut has_len_more_than_one = false;
            for element in &read_only_ids_to_two_dimensional_vec_read_inner {
                if element.len() > 1 {
                    has_len_more_than_one = true;
                    break;
                }
            }
            has_len_more_than_one
        };
        acc.push(vec![Some({
            let mut acc = vec![];
            for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                for element1 in element0 {
                    acc.push(element1);
                }
            }
            acc
        })]);
        acc.push(vec![None]);
        if let Some(value) = option_additional {
            if has_len_more_than_one {
                acc.push(value.0);
            }
            if !has_len_more_than_one {
                acc.push(value.1);
            }
        }
        acc
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(
                <<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            ),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        match create.0 .0 {
            Some(value) => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(Some(
                postgresql_crud_common::NotEmptyUniqueEnumVec::try_new(vec![VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
                    logical_operator: postgresql_crud_common::LogicalOperator::Or,
                    value: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration::new(value.into()),
                })])
                .expect("error 88bfa095-a3ab-4d0c-be71-af63c3acd50f"),
            )),
            None => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(None),
        }
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some(vec![match create.0 .0 {
            Some(value) => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(Some(
                postgresql_crud_common::NotEmptyUniqueEnumVec::try_new(
                    <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
                        VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None }),
                        VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreate(value),
                    )
                    .expect("error f710c474-35ab-4d01-8810-596f280bbaab"),
                )
                .expect("error 05414930-9f98-4ae9-b925-3be906e95329"),
            )),
            None => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(None),
        }])
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        None
    }
}
#[derive(Debug)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin(std::vec::Vec<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin>);
impl VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    pub fn new(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(value.into_iter().map(|element| OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin::new(element)).collect())
    }
}
impl std::convert::From<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreate> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn from(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn into(self) -> VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner {
        self.0
            .into_iter()
            .map(|element| match element.0 {
                Some(value) => Some(value.0.into_iter().map(|element| element.0).collect()),
                None => None,
            })
            .collect()
    }
}
impl std::convert::From<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdate> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn from(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl std::fmt::Display for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberTableTypeDeclaration(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberTableTypeDeclaration {
    pub fn new(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreate(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreate {
    pub fn new(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreateForQuery(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreateForQuery {
    pub fn new(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl std::convert::From<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreate> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreateForQuery {
    fn from(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect {
    dimension1_pagination: postgresql_crud_common::PaginationStartsWithZero,
    dimension2_pagination: postgresql_crud_common::PaginationStartsWithZero,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimension2_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
            dimension2_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
    Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration>),
    DimensionTwoEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoEqual<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneLengthEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthEqual),
    DimensionTwoLengthEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoLengthEqual),
    DimensionOneLengthMoreThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthMoreThan),
    DimensionTwoLengthMoreThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoLengthMoreThan),
    DimensionTwoContainsAllElementsOfArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoContainsAllElementsOfArray<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionTwoOverlapsWithArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoOverlapsWithArray<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    AllElementsEqual(where_element_filters::PostgresqlJsonTypeWhereElementAllElementsEqual<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneAllElementsEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneAllElementsEqual<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneIn(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneIn<OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration>),
    DimensionTwoIn(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoIn<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionTwoGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionTwoBetween(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoBetween<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneContainsElementGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneContainsElementGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
    DimensionOneAllElementsGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneAllElementsGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration>),
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneAllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneAllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoContainsAllElementsOfArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoOverlapsWithArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAllElementsEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoBetween(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneContainsElementGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAllElementsGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn new(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner = std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdate(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdate {
    pub fn new(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdateForQuery(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdateForQuery {
    pub fn new(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdate> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdateForQuery {
    fn from(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber {
    type TableTypeDeclaration = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberTableTypeDeclaration;
    type Create = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreate;
    type CreateForQuery = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberCreateForQuery;
    type Select = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        let dimension2_start = value.dimension2_pagination.start();
        let dimension2_end = value.dimension2_pagination.end();
        format ! ("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg((case when jsonb_typeof(d1_elem.value)='array' then (select jsonb_agg((d2_elem.value)) from jsonb_array_elements((d1_elem.value)) with ordinality as d2_elem(value, d2_elem) where d2_elem between {dimension2_start} and {dimension2_end}) else null end)) from jsonb_array_elements(({column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality as d1_elem(value, d1_elem) where d1_elem between {dimension1_start} and {dimension1_end})))")
    }
    type WhereElement = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement;
    type Read = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead;
    type ReadOnlyIds = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value
            .0
             .0
            .into_iter()
            .map(|element| match element.0 {
                Some(value) => Some(value.0.into_iter().map(|element| element.0).collect()),
                None => None,
            })
            .collect()
    }
    type Update = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdate;
    type UpdateForQuery = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        let mut acc = vec![];
        for element in <OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
            acc.push(<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(vec![element.0.into()]));
        }
        acc.push(<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new({
            let mut acc = vec![];
            for element in <OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
                acc.push(element.0.into());
            }
            acc
        }));
        acc
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        let read_only_ids_to_two_dimensional_vec_read_inner = <OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberReadOnlyIds(read_only_ids.0.clone()));
        let option_additional = {
            let mut option_additional = None;
            for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                if option_additional.is_some() {
                    break;
                }
                for element1 in element0 {
                    if option_additional.is_none() {
                        option_additional = Some((vec![vec![element1.clone()]], vec![vec![element1.clone(), element1.clone()]]));
                    } else {
                        break;
                    }
                }
            }
            option_additional
        };
        let has_len_more_than_one = read_only_ids_to_two_dimensional_vec_read_inner.len() > 1;
        acc.push(vec![{
            let mut acc = vec![];
            for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                for element1 in element0 {
                    acc.push(element1);
                }
            }
            acc
        }]);
        if let Some(value) = option_additional {
            if has_len_more_than_one {
                acc.push(value.0);
            }
            if !has_len_more_than_one {
                acc.push(value.1);
            }
        }
        acc
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(
                <<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            ),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
            logical_operator: postgresql_crud_common::LogicalOperator::Or,
            value: VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberTableTypeDeclaration::new(create.0.into()),
        })
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0 .0.into_iter().enumerate() {
                acc.push(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement::DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual {
                    logical_operator: postgresql_crud_common::LogicalOperator::And,
                    dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                    value: OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberTableTypeDeclaration(element),
                }));
            }
            acc
        })
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0 .0.into_iter().enumerate() {
                if let Some(value) = element.0 {
                    for element in value.0 {
                        acc.push(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberWhereElement::DimensionTwoEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoEqual {
                            logical_operator: postgresql_crud_common::LogicalOperator::And,
                            dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                            value: StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration::new(element.into()),
                        }));
                    }
                }
            }
            acc
        })
    }
}
#[derive(Debug)]
pub struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin(std::option::Option<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin>);
impl OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin {
    pub fn new(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(match value {
            Some(value) => Some(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin::new(value)),
            None => None,
        })
    }
}
impl std::convert::From<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreate> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin {
    fn from(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin {
    fn into(self) -> OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner {
        match self.0 {
            Some(value) => Some(
                value
                    .0
                    .into_iter()
                    .map(|element| match element.0 {
                        Some(value) => Some(value.0),
                        None => None,
                    })
                    .collect(),
            ),
            None => None,
        }
    }
}
impl std::convert::From<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdate> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin {
    fn from(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl std::fmt::Display for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration {
    pub fn new(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreate(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreate {
    pub fn new(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreateForQuery(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreateForQuery {
    pub fn new(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl std::convert::From<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreate> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreateForQuery {
    fn from(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberSelect {
    dimension1_pagination: postgresql_crud_common::PaginationStartsWithZero,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
        }
    }
}
pub type OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberWhereElement = postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter<<VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::WhereElement>;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberRead {
    pub fn new(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner = std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdate(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdate {
    pub fn new(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdateForQuery(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdateForQuery {
    pub fn new(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdate> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdateForQuery {
    fn from(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber {
    type TableTypeDeclaration = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration;
    type Create = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreate;
    type CreateForQuery = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberCreateForQuery;
    type Select = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}')='null' then 'null'::jsonb else (select jsonb_agg((value)) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {dimension1_start} and {dimension1_end}) end)))")
    }
    type WhereElement = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberWhereElement;
    type Read = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberRead;
    type ReadOnlyIds = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(
                value
                    .0
                    .into_iter()
                    .map(|element| match element.0 {
                        Some(value) => Some(value.0),
                        None => None,
                    })
                    .collect(),
            ),
            None => None,
        }
    }
    type Update = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdate;
    type UpdateForQuery = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        let mut acc = vec![];
        for element in <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
            acc.push(<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(Some(element.0.into())));
        }
        acc.push(<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(None));
        acc
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        let read_only_ids_to_two_dimensional_vec_read_inner = <OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&OptionStdPrimitiveI8AsNullableJsonbNumberReadOnlyIds(read_only_ids.0.clone()));
        let option_additional = {
            let mut option_additional = None;
            for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                if option_additional.is_some() {
                    break;
                }
                for element1 in element0 {
                    if option_additional.is_none() {
                        option_additional = Some((vec![Some(vec![element1.clone()])], vec![Some(vec![element1.clone(), element1.clone()])]));
                    } else {
                        break;
                    }
                }
            }
            option_additional
        };
        let has_len_more_than_one = read_only_ids_to_two_dimensional_vec_read_inner.len() > 1;
        acc.push(vec![Some({
            let mut acc = vec![];
            for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                for element1 in element0 {
                    acc.push(element1);
                }
            }
            acc
        })]);
        acc.push(vec![None]);
        if let Some(value) = option_additional {
            if has_len_more_than_one {
                acc.push(value.0);
            }
            if !has_len_more_than_one {
                acc.push(value.1);
            }
        }
        acc
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(
                <<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            ),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        match create.0 .0 {
            Some(value) => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(Some(
                postgresql_crud_common::NotEmptyUniqueEnumVec::try_new(vec![VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
                    logical_operator: postgresql_crud_common::LogicalOperator::Or,
                    value: VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberTableTypeDeclaration::new(value.into()),
                })])
                .expect("error 88bfa095-a3ab-4d0c-be71-af63c3acd50f"),
            )),
            None => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(None),
        }
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some(vec![match create.0 .0 {
            Some(value) => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(Some(
                postgresql_crud_common::NotEmptyUniqueEnumVec::try_new(
                    <VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
                        VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None }),
                        VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberCreate(value),
                    )
                    .expect("error f710c474-35ab-4d01-8810-596f280bbaab"),
                )
                .expect("error 05414930-9f98-4ae9-b925-3be906e95329"),
            )),
            None => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(None),
        }])
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        None
    }
}
#[derive(Debug)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin(std::vec::Vec<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin>);
impl VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin {
    pub fn new(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(value.into_iter().map(|element| OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin::new(element)).collect())
    }
}
impl std::convert::From<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreate> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin {
    fn from(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin {
    fn into(self) -> VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner {
        self.0
            .into_iter()
            .map(|element| match element.0 {
                Some(value) => Some(
                    value
                        .0
                        .into_iter()
                        .map(|element| match element.0 {
                            Some(value) => Some(value.0),
                            None => None,
                        })
                        .collect(),
                ),
                None => None,
            })
            .collect()
    }
}
impl std::convert::From<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdate> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin {
    fn from(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl std::fmt::Display for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberTableTypeDeclaration(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberTableTypeDeclaration {
    pub fn new(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreate(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreate {
    pub fn new(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreateForQuery(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreateForQuery {
    pub fn new(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl std::convert::From<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreate> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreateForQuery {
    fn from(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberSelect {
    dimension1_pagination: postgresql_crud_common::PaginationStartsWithZero,
    dimension2_pagination: postgresql_crud_common::PaginationStartsWithZero,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimension2_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
            dimension2_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberWhereElement {
    Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration>),
    DimensionTwoEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoEqual<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneLengthEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthEqual),
    DimensionTwoLengthEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoLengthEqual),
    DimensionOneLengthMoreThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneLengthMoreThan),
    DimensionTwoLengthMoreThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoLengthMoreThan),
    DimensionTwoContainsAllElementsOfArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoContainsAllElementsOfArray<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionTwoOverlapsWithArray(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoOverlapsWithArray<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    AllElementsEqual(where_element_filters::PostgresqlJsonTypeWhereElementAllElementsEqual<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneAllElementsEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneAllElementsEqual<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneIn(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneIn<OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration>),
    DimensionTwoIn(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoIn<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionTwoGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoGreaterThan<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionTwoBetween(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoBetween<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneContainsElementGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneContainsElementGreaterThan<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
    DimensionOneAllElementsGreaterThan(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneAllElementsGreaterThan<OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration>),
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneAllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionTwoBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneAllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Equal(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoLengthEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoLengthMoreThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoContainsAllElementsOfArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoOverlapsWithArray(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::AllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAllElementsEqual(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoIn(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionTwoBetween(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneContainsElementGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAllElementsGreaterThan(value) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoLengthEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoLengthMoreThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoContainsAllElementsOfArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoOverlapsWithArray(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAllElementsEqual(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoIn(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionTwoBetween(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneContainsElementGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAllElementsGreaterThan(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn new(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner = std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdate(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdate {
    pub fn new(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdateForQuery(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdateForQuery {
    pub fn new(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner) -> Self {
        Self(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdate> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdateForQuery {
    fn from(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber {
    type TableTypeDeclaration = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberTableTypeDeclaration;
    type Create = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreate;
    type CreateForQuery = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberCreateForQuery;
    type Select = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        let dimension2_start = value.dimension2_pagination.start();
        let dimension2_end = value.dimension2_pagination.end();
        format ! ("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg((case when jsonb_typeof(d1_elem.value)='array' then (select jsonb_agg((d2_elem.value)) from jsonb_array_elements((d1_elem.value)) with ordinality as d2_elem(value, d2_elem) where d2_elem between {dimension2_start} and {dimension2_end}) else null end)) from jsonb_array_elements(({column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality as d1_elem(value, d1_elem) where d1_elem between {dimension1_start} and {dimension1_end})))")
    }
    type WhereElement = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberWhereElement;
    type Read = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead;
    type ReadOnlyIds = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value
            .0
             .0
            .into_iter()
            .map(|element| match element.0 {
                Some(value) => Some(
                    value
                        .0
                        .into_iter()
                        .map(|element| match element.0 {
                            Some(value) => Some(value.0),
                            None => None,
                        })
                        .collect(),
                ),
                None => None,
            })
            .collect()
    }
    type Update = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdate;
    type UpdateForQuery = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        let mut acc = vec![];
        for element in <OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
            acc.push(<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(vec![element.0.into()]));
        }
        acc.push(<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new({
            let mut acc = vec![];
            for element in <OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
                acc.push(element.0.into());
            }
            acc
        }));
        acc
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        let read_only_ids_to_two_dimensional_vec_read_inner = <OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberReadOnlyIds(read_only_ids.0.clone()));
        let option_additional = {
            let mut option_additional = None;
            for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                if option_additional.is_some() {
                    break;
                }
                for element1 in element0 {
                    if option_additional.is_none() {
                        option_additional = Some((vec![vec![element1.clone()]], vec![vec![element1.clone(), element1.clone()]]));
                    } else {
                        break;
                    }
                }
            }
            option_additional
        };
        let has_len_more_than_one = read_only_ids_to_two_dimensional_vec_read_inner.len() > 1;
        acc.push(vec![{
            let mut acc = vec![];
            for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                for element1 in element0 {
                    acc.push(element1);
                }
            }
            acc
        }]);
        if let Some(value) = option_additional {
            if has_len_more_than_one {
                acc.push(value.0);
            }
            if !has_len_more_than_one {
                acc.push(value.1);
            }
        }
        acc
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(
                <<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            ),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
            logical_operator: postgresql_crud_common::LogicalOperator::Or,
            value: VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberTableTypeDeclaration::new(create.0.into()),
        })
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0 .0.into_iter().enumerate() {
                acc.push(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberWhereElement::DimensionOneEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual {
                    logical_operator: postgresql_crud_common::LogicalOperator::And,
                    dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                    value: OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberTableTypeDeclaration(element),
                }));
            }
            acc
        })
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0 .0.into_iter().enumerate() {
                if let Some(value) = element.0 {
                    for element in value.0 {
                        acc.push(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberWhereElement::DimensionTwoEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoEqual {
                            logical_operator: postgresql_crud_common::LogicalOperator::And,
                            dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                            value: OptionStdPrimitiveI8AsNullableJsonbNumberTableTypeDeclaration::new(element.into()),
                        }));
                    }
                }
            }
            acc
        })
    }
}
#[derive(Debug)]
pub struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin(std::option::Option<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin>);
impl OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    pub fn new(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(match value {
            Some(value) => Some(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value)),
            None => None,
        })
    }
}
impl std::convert::From<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreate> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn from(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn into(self) -> OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner {
        match self.0 {
            Some(value) => Some(value.0.into_iter().map(|element| element.0.into_iter().map(|element| element.0).collect()).collect()),
            None => None,
        }
    }
}
impl std::convert::From<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdate> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn from(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdate) -> Self {
        value.0
    }
}
impl std::fmt::Display for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    pub fn new(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreate(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreate {
    pub fn new(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    pub fn new(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl std::convert::From<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreate> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery {
    fn from(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreate) -> Self {
        Self(value.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberSelect {
    dimension1_pagination: postgresql_crud_common::PaginationStartsWithZero,
    dimension2_pagination: postgresql_crud_common::PaginationStartsWithZero,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimension2_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self {
        Self {
            dimension1_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
            dimension2_pagination: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(),
        }
    }
}
pub type OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement = postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter<<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::WhereElement>;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn new(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadOnlyIds(pub postgresql_crud_common::Value<std::option::Option<()>>);
pub type OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner = std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdate(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdate {
    pub fn new(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    pub fn new(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> Self {
        Self(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin::new(value))
    }
}
impl std::convert::From<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdate> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    fn from(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdate) -> Self {
        Self(value.0)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl postgresql_crud_common::PostgresqlJsonType for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber {
    type TableTypeDeclaration = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration;
    type Create = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreate;
    type CreateForQuery = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberCreateForQuery;
    type Select = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        let dimension2_start = value.dimension2_pagination.start();
        let dimension2_end = value.dimension2_pagination.end();
        format ! ("jsonb_build_object('{field_ident}',jsonb_build_object('value',(case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}')='null' then 'null'::jsonb else (select jsonb_agg((select jsonb_agg((d2_elem.value)) from jsonb_array_elements((d1_elem.value)) with ordinality as d2_elem(value, d2_elem) where d2_elem between {dimension2_start} and {dimension2_end})) from jsonb_array_elements(({column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality as d1_elem(value, d1_elem) where d1_elem between {dimension1_start} and {dimension1_end}) end)))")
    }
    type WhereElement = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement;
    type Read = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead;
    type ReadOnlyIds = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        "jsonb_build_object('value','null'::jsonb)".to_string()
    }
    type ReadInner = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(value.0.into_iter().map(|element| element.0.into_iter().map(|element| element.0).collect()).collect()),
            None => None,
        }
    }
    type Update = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdate;
    type UpdateForQuery = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        ok_field_ident_jsonb_build_object_value(&field_ident)
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber {
    type PostgresqlJsonType = Self;
    type Select = OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        let mut acc = vec![];
        for element in <VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::vec_create() {
            acc.push(<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(Some(element.0.into())));
        }
        acc.push(<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Create::new(None));
        acc
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        let read_only_ids_to_two_dimensional_vec_read_inner = <VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadOnlyIds(read_only_ids.0.clone()));
        let option_additional = {
            let mut option_additional = None;
            for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                if option_additional.is_some() {
                    break;
                }
                for element1 in element0 {
                    if option_additional.is_none() {
                        option_additional = Some((vec![Some(vec![element1.clone()])], vec![Some(vec![element1.clone(), element1.clone()])]));
                    } else {
                        break;
                    }
                }
            }
            option_additional
        };
        let has_len_more_than_one = read_only_ids_to_two_dimensional_vec_read_inner.len() > 1;
        acc.push(vec![Some({
            let mut acc = vec![];
            for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                for element1 in element0 {
                    acc.push(element1);
                }
            }
            acc
        })]);
        acc.push(vec![None]);
        if let Some(value) = option_additional {
            if has_len_more_than_one {
                acc.push(value.0);
            }
            if !has_len_more_than_one {
                acc.push(value.1);
            }
        }
        acc
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value {
            value: <OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::into_inner(
                <<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::Read as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            ),
        })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(create.into())
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration(create.into())
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        match create.0 .0 {
            Some(value) => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(Some(
                postgresql_crud_common::NotEmptyUniqueEnumVec::try_new(vec![VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
                    logical_operator: postgresql_crud_common::LogicalOperator::Or,
                    value: VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberTableTypeDeclaration::new(value.into()),
                })])
                .expect("error 88bfa095-a3ab-4d0c-be71-af63c3acd50f"),
            )),
            None => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(None),
        }
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some(vec![match create.0 .0 {
            Some(value) => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(Some(
                postgresql_crud_common::NotEmptyUniqueEnumVec::try_new(
                    <VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
                        VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberReadOnlyIds(postgresql_crud_common::Value { value: None }),
                        VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberCreate(value),
                    )
                    .expect("error f710c474-35ab-4d01-8810-596f280bbaab"),
                )
                .expect("error 05414930-9f98-4ae9-b925-3be906e95329"),
            )),
            None => postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(None),
        }])
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        Some({
            let 
            mut 
            acc = vec![];
            //
            if let Some(value) = create.0.0 {
                for (index, element) in value.0.into_iter().enumerate() {
                    for element in element.0 {
                        //
                        // postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter<<VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber as postgresql_crud_common::PostgresqlJsonType>::WhereElement>;

                        // let f: bool = element;
                        // NullableJsonObjectPostgresqlTypeWhereFilter(pub std::option::Option<NotEmptyUniqueEnumVec<T>>);
                        if let Ok(value) = postgresql_crud_common::NotEmptyUniqueEnumVec::try_new(
                            vec![
                                true
                            ]
                        ) {
                            acc.push(postgresql_crud_common::NullableJsonObjectPostgresqlTypeWhereFilter(Some(
                                value
                            )));
                        }

                            
                            
                            // ::DimensionTwoEqual(where_element_filters::PostgresqlJsonTypeWhereElementDimensionTwoEqual {
                            //     logical_operator: postgresql_crud_common::LogicalOperator::And,
                            //     dimensions: where_element_filters::BoundedStdVecVec::try_from(vec![where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(std::primitive::i32::try_from(index).expect("error 5341936f-ce9e-4e14-ae30-765f04c12e14")).expect("error 76906f3c-4472-4ac0-a605-1b02f02fd680")]).expect("error 8a624c70-3701-4907-b361-5637c5361e1f"),
                            //     value: StdPrimitiveI8AsNotNullJsonbNumberTableTypeDeclaration::new(element.into()),
                            // })
                    }
                }
            }
            acc
        })
    }
}
#[derive(Debug)]
pub struct UuidUuidAsNotNullJsonbString;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
struct UuidUuidAsNotNullJsonbStringOrigin(uuid::Uuid);
impl UuidUuidAsNotNullJsonbStringOrigin {
    pub fn new(value: UuidUuidAsNotNullJsonbStringReadInner) -> Self {
        Self(value)
    }
}
impl std::convert::From<UuidUuidAsNotNullJsonbStringCreate> for UuidUuidAsNotNullJsonbStringOrigin {
    fn from(value: UuidUuidAsNotNullJsonbStringCreate) -> Self {
        value.0
    }
}
impl std::convert::Into<UuidUuidAsNotNullJsonbStringReadInner> for UuidUuidAsNotNullJsonbStringOrigin {
    fn into(self) -> UuidUuidAsNotNullJsonbStringReadInner {
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
        <sqlx::types::Json<UuidUuidAsNotNullJsonbStringReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<UuidUuidAsNotNullJsonbStringReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UuidUuidAsNotNullJsonbStringOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(&self.0), buf)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringTableTypeDeclaration(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringTableTypeDeclaration {
    pub fn new(value: UuidUuidAsNotNullJsonbStringReadInner) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UuidUuidAsNotNullJsonbStringTableTypeDeclaration {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for UuidUuidAsNotNullJsonbStringTableTypeDeclaration {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<UuidUuidAsNotNullJsonbStringReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<UuidUuidAsNotNullJsonbStringReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringCreate(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringCreate {
    pub fn new(value: UuidUuidAsNotNullJsonbStringReadInner) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct UuidUuidAsNotNullJsonbStringCreateForQuery(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringCreateForQuery {
    pub fn new(value: UuidUuidAsNotNullJsonbStringReadInner) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
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
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum UuidUuidAsNotNullJsonbStringWhereElement {
    Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual<UuidUuidAsNotNullJsonbStringTableTypeDeclaration>),
    In(where_element_filters::PostgresqlJsonTypeWhereElementIn<UuidUuidAsNotNullJsonbStringTableTypeDeclaration>),
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
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringRead(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringRead {
    pub fn new(value: UuidUuidAsNotNullJsonbStringReadInner) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidAsNotNullJsonbStringRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UuidUuidAsNotNullJsonbStringRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for UuidUuidAsNotNullJsonbStringRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<UuidUuidAsNotNullJsonbStringReadInner> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<UuidUuidAsNotNullJsonbStringReadInner> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UuidUuidAsNotNullJsonbStringReadOnlyIds(pub postgresql_crud_common::Value<uuid::Uuid>);
pub type UuidUuidAsNotNullJsonbStringReadInner = uuid::Uuid;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringUpdate(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringUpdate {
    pub fn new(value: UuidUuidAsNotNullJsonbStringReadInner) -> Self {
        Self(UuidUuidAsNotNullJsonbStringOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for UuidUuidAsNotNullJsonbStringUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
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
    pub fn new(value: UuidUuidAsNotNullJsonbStringReadInner) -> Self {
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
        value.0 .0
    }
    type Update = UuidUuidAsNotNullJsonbStringUpdate;
    type UpdateForQuery = UuidUuidAsNotNullJsonbStringUpdateForQuery;
    fn update_query_part(_: &Self::UpdateForQuery, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind(value: Self::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::UpdateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("'{field_ident}',jsonb_build_object('value',${value}),")),
            Err(error) => Err(error),
        }
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::UpdateForQuery, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn select_only_created_ids_query_part(value: &Self::CreateForQuery, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("'{field_ident}',jsonb_build_object('value',${value}),")),
            Err(error) => Err(error),
        }
    }
    fn select_only_created_ids_query_bind<'a>(value: &'a Self::CreateForQuery, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
impl postgresql_crud_common::PostgresqlJsonTypeObjectVecElementId for UuidUuidAsNotNullJsonbString {
    type PostgresqlJsonType = UuidUuidAsNotNullJsonbString;
    type CreateForQuery = UuidUuidAsNotNullJsonbStringCreateForQuery;
    type Update = UuidUuidAsNotNullJsonbStringUpdate;
    type ReadInner = UuidUuidAsNotNullJsonbStringReadInner;
    fn query_bind_string_as_postgresql_text_create_for_query(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::CreateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value.0 .0.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn query_bind_string_as_postgresql_text_update_for_query(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::UpdateForQuery, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(value.0 .0.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn get_inner<'a>(value: &'a <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::CreateForQuery) -> &'a Self::ReadInner {
        &value.0 .0
    }
    fn increment_checked_add_one(increment: &mut std::primitive::u64) -> Result<std::primitive::u64, postgresql_crud_common::QueryPartErrorNamed> {
        postgresql_crud_common::increment_checked_add_one_returning_increment(increment)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud_common::PostgresqlJsonTypeTestCases for UuidUuidAsNotNullJsonbString {
    type PostgresqlJsonType = Self;
    type Select = UuidUuidAsNotNullJsonbStringSelect;
    fn vec_create() -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create> {
        vec![]
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(read_only_ids: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        vec![]
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(value: UuidUuidAsNotNullJsonbStringReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read::new(value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(value: UuidUuidAsNotNullJsonbStringReadInner) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update {
        <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update::new(value)
    }
    fn read_only_ids_into_option_value_read_inner(value: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud_common::Value { value: value.0.value })
    }
    fn update_to_read_only_ids(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds {
        UuidUuidAsNotNullJsonbStringReadOnlyIds(postgresql_crud_common::Value { value: value.0.clone().into() })
    }
    fn read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value: &<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value { value: UuidUuidAsNotNullJsonbStringRead::new(value.0.value.clone()) })
    }
    fn previous_read_merged_with_option_update_into_read(read: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read, option_update: std::option::Option<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Update>) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        match option_update {
            Some(value) => UuidUuidAsNotNullJsonbStringRead(value.into()),
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read {
        UuidUuidAsNotNullJsonbStringRead(UuidUuidAsNotNullJsonbStringOrigin::new(read_only_ids.0.value))
    }
    fn read_only_ids_merged_with_create_into_option_value_read(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::option::Option<postgresql_crud_common::Value<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Read>> {
        Some(postgresql_crud_common::Value {
            value: <UuidUuidAsNotNullJsonbString as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_read(read_only_ids, create),
        })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::TableTypeDeclaration {
        UuidUuidAsNotNullJsonbStringTableTypeDeclaration(UuidUuidAsNotNullJsonbStringOrigin::new(read_only_ids.0.value))
    }
    fn read_only_ids_merged_with_create_into_where_element_equal(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement {
        UuidUuidAsNotNullJsonbStringWhereElement::Equal(where_element_filters::PostgresqlJsonTypeWhereElementEqual {
            logical_operator: postgresql_crud_common::LogicalOperator::Or,
            value: UuidUuidAsNotNullJsonbStringTableTypeDeclaration::new(create.0.into()),
        })
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        vec![<UuidUuidAsNotNullJsonbString as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_where_element_equal(read_only_ids, create)]
    }
    fn read_only_ids_merged_with_create_into_vec_where_element_equal_to_json_field(read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds, create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create) -> std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement> {
        <UuidUuidAsNotNullJsonbString as postgresql_crud_common::PostgresqlJsonTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_element_equal_using_fields(read_only_ids, create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        None
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::Create,
    ) -> std::option::Option<std::vec::Vec<<Self::PostgresqlJsonType as postgresql_crud_common::PostgresqlJsonType>::WhereElement>> {
        todo!()
    }
}
