generate_postgresql_json_types::generate_postgresql_json_types!([
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "Nullable",
        "postgresql_json_type_pattern": "Standart"
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "Nullable",
        "postgresql_json_type_pattern": {
            "ArrayDimension4": {
                "dimension1_not_null_or_nullable": "Nullable",
                "dimension2_not_null_or_nullable": "Nullable",
                "dimension3_not_null_or_nullable": "Nullable",
                "dimension4_not_null_or_nullable": "Nullable"
            }
        }
    },
    {
        "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    },
    {
        "postgresql_json_type": "UuidUuidAsJsonbString",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    }
]);