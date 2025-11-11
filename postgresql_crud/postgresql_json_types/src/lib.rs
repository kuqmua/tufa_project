// generate_postgresql_json_types::generate_postgresql_json_types!("All");
generate_postgresql_json_types::generate_postgresql_json_types!({
    "Concrete": [
        {
            "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
            "not_null_or_nullable": "NotNull",
            "postgresql_json_type_pattern": "Standart"
        }
        ,
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": "Standart"
        // }
        // ,
        {
            "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
            "not_null_or_nullable": "NotNull",
            "postgresql_json_type_pattern": {
                "ArrayDimension1": {
                    "dimension1_not_null_or_nullable": "NotNull"
                }
            }
        },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveI64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU8AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU16AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveU64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF32AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveF64AsJsonbNumber",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdPrimitiveBoolAsJsonbBoolean",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": "Standart"
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension1": {
//                     "dimension1_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension2": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension3": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "NotNull",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "NotNull",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "NotNull",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "NotNull"
//                 }
//             }
//         },
//         {
//             "postgresql_json_type": "StdStringStringAsJsonbString",
//             "not_null_or_nullable": "Nullable",
//             "postgresql_json_type_pattern": {
//                 "ArrayDimension4": {
//                     "dimension1_not_null_or_nullable": "Nullable",
//                     "dimension2_not_null_or_nullable": "Nullable",
//                     "dimension3_not_null_or_nullable": "Nullable",
//                     "dimension4_not_null_or_nullable": "Nullable"
//                 }
//             }
//         },
        {
            "postgresql_json_type": "UuidUuidAsJsonbString",
            "not_null_or_nullable": "NotNull",
            "postgresql_json_type_pattern": "Standart"
        }
        // ,
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": "Standart"
        // }
        // ,
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // }
        // ,
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension2": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension3": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "NotNull",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "NotNull",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "NotNull",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_json_type": "UuidUuidAsJsonbString",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_json_type_pattern": {
        //         "ArrayDimension4": {
        //             "dimension1_not_null_or_nullable": "Nullable",
        //             "dimension2_not_null_or_nullable": "Nullable",
        //             "dimension3_not_null_or_nullable": "Nullable",
        //             "dimension4_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // }
    ]
});

fn ok_field_ident_jsonb_build_object_value(field_ident: &std::primitive::str) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
    Ok(format!("'{field_ident}',jsonb_build_object('value','null'::jsonb),"))
}

////////
//here
impl VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber {
    pub fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_element_dimension_one_equal(
        read_only_ids: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberReadOnlyIds,//todo maybe rot need
        create: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberCreate
    ) -> std::option::Option<std::vec::Vec<VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberWhereElement>> {
        Some({
            let mut acc = vec![];
            for (index, element) in create.0.0.into_iter().enumerate() {
                acc.push(
                    VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberWhereElement::DimensionOneEqual(
                        where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual {
                            logical_operator: postgresql_crud_common::LogicalOperator::Or,
                            dimensions: where_element_filters::BoundedStdVecVec::try_from(
                                vec![
                                    where_element_filters::UnsignedPartOfStdPrimitiveI32::try_from(
                                        std::primitive::i32::try_from(index).expect("error 9c9e7aa5-09fb-48f8-be51-f6b739942130")
                                    ).unwrap()
                                ]
                            ).unwrap(),
                            value: element,
                        }
                    )
                )
            }
            acc
        })
    }
}