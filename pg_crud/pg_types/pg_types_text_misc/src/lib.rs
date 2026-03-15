gen_pg_types::gen_pg_types!({
    "pg_tbl_cols_write_into_file": "False",
    "whole_write_into_file": "False",
    "vrt": {
        "Subset": [
            "StringAsText",
            "StdVecVecU8AsBytea",
            "SqlxTypesUuidUuidAsUuidV4InitByPg",
            "SqlxTypesUuidUuidAsUuidInitByClient",
            "SqlxTypesTimeTimeAsTime",
            "SqlxPgTypesPgIntervalAsInterval"
        ]
    }
});
