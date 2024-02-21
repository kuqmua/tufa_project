pub use ::naming_constants::COMMIT;
pub use generate_postgresql_crud::additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::create_many_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::create_one_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::delete_many_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::delete_one_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::read_many_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::read_one_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::update_many_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::update_one_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::GeneratePostgresqlCrud;

pub mod app_state;
pub mod json_value_extractor;
pub struct Test<T> {
    //https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
    std_primitive_bool: std::primitive::bool, //BOOL
    std_primitive_i8: std::primitive::i8,   //“CHAR”
    std_primitive_i16: std::primitive::i16,  //SMALLINT, SMALLSERIAL, INT2
    std_primitive_i32: std::primitive::i32,  //INT, SERIAL, INT4
    std_primitive_i64: std::primitive::i64,  //BIGINT, BIGSERIAL, INT8
    std_primitive_f32: std::primitive::f32,  //REAL, FLOAT4
    std_primitive_f64: std::primitive::f64,  //DOUBLE PRECISION, FLOAT8
    // type_8: &std::primitive::str,//lifetimes are unexpectable i think //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    std_string_string: std::string::String, //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    // type_10: [std::primitive::u8;1],//ignoring coz deserialization problem//BYTEA
    std_vec_vec_std_primitive_u8: std::vec::Vec<std::primitive::u8>, //BYTEA
    // type_12: (),//didnt find Encode trait impl in sqlx//BYTEA
    sqlx_postgres_types_pg_interval: sqlx::postgres::types::PgInterval, //INTERVAL
    //INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
    sqlx_postgres_types_pg_range_std_primitive_i64: sqlx::postgres::types::PgRange<std::primitive::i64>, //INT8RANGE
    sqlx_postgres_types_pg_range_std_primitive_i32: sqlx::postgres::types::PgRange<std::primitive::i32>, //INT4RANGE
    // type_16: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc:
        sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>, //TSRANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>, //maybe not correct//TSRANGE
    // type_17: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>>, //TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>, //TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>, //maybe not correct//TSTZRANGE
    // type_18: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//DATERANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>, //maybe not correct//DATERANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_date: sqlx::postgres::types::PgRange<sqlx::types::time::Date>, //maybe not correct//DATERANGE
    // type_19: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//NUMRANGE
    sqlx_postgres_types_pg_range_sqlx_types_big_decimal: sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>, //NUMRANGE
    sqlx_postgres_types_pg_range_sqlx_types_decimal: sqlx::postgres::types::PgRange<sqlx::types::Decimal>,    //NUMRANGE
    sqlx_postgres_types_pg_money: sqlx::postgres::types::PgMoney,                           //MONEY
    sqlx_postgres_types_pg_l_tree: sqlx::postgres::types::PgLTree,                           //LTREE
    sqlx_postgres_types_pg_l_query: sqlx::postgres::types::PgLQuery,                          //LQUERY
    sqlx_postgres_types_pg_ci_text: sqlx::postgres::types::PgCiText,                          //CITEXT
    sqlx_types_big_decimal: sqlx::types::BigDecimal,                                  //NUMERIC
    sqlx_types_decimal: sqlx::types::Decimal,                                     //NUMERIC
    sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset: sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>, //TIMESTAMPTZ
    sqlx_types_chrono_date_time_sqlx_types_chrono_local: sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>, //TIMESTAMPTZ
    sqlx_types_chrono_date_time_sqlx_types_chrono_utc: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,  //TIMESTAMP
    sqlx_types_chrono_naive_date_time: sqlx::types::chrono::NaiveDateTime,//TIMESTAMP
    sqlx_types_chrono_naive_date: sqlx::types::chrono::NaiveDate,                           //DATE
    sqlx_types_chrono_naive_time: sqlx::types::chrono::NaiveTime,                           //TIME
    sqlx_postgres_types_pg_time_tz: sqlx::postgres::types::PgTimeTz, //just present chrono or time flag
    // type_: sqlx::postgres::types::PgTimeTz,//feature flag chrono//TIMETZ
    sqlx_types_time_primitive_date_time: sqlx::types::time::PrimitiveDateTime, //TIMESTAMP
    sqlx_types_time_offset_date_time: sqlx::types::time::OffsetDateTime,    //TIMESTAMPTZ
    sqlx_types_time_date: sqlx::types::time::Date,              //DATE
    sqlx_types_time_time: sqlx::types::time::Time,              //TIME
    // type_: sqlx::postgres::types::PgTimeTz,//feature flag time//TIMETZ
    sqlx_types_uuid_uuid: sqlx::types::uuid::Uuid,              //UUID
    sqlx_types_ipnetwork_ip_network: sqlx::types::ipnetwork::IpNetwork,    //INET, CIDR
    std_net_ip_addr: std::net::IpAddr,                     //INET, CIDR
    sqlx_types_mac_address_mac_address: sqlx::types::mac_address::MacAddress, //MACADDR
    sqlx_types_bit_vec: sqlx::types::BitVec,                  //BIT, VARBIT
    sqlx_types_json: sqlx::types::Json<T>,                 //JSON, JSONB
    serde_json_value: serde_json::Value,                    //JSON, JSONB
                                                   // type_44: serde_json::value::RawValue,//lifetime and borrow problem//JSON, JSONB
                                                   //maybe Composite types
                                                   //maybe Enumerations
}

pub struct TestNewType<T> {
    //https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
    std_primitive_bool: StdPrimitiveBool, //BOOL
    std_primitive_i8: StdPrimitiveI8,   //“CHAR”
    std_primitive_i16: StdPrimitiveI16,  //SMALLINT, SMALLSERIAL, INT2
    std_primitive_i32: StdPrimitiveI32,  //INT, SERIAL, INT4
    std_primitive_i64: StdPrimitiveI64,  //BIGINT, BIGSERIAL, INT8
    std_primitive_f32: StdPrimitiveF32,  //REAL, FLOAT4
    std_primitive_f64: StdPrimitiveF64,  //DOUBLE PRECISION, FLOAT8
    // type_8: &std::primitive::str,//lifetimes are unexpectable i think //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    std_string_string: StdStringString, //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    // type_10: [std::primitive::u8;1],//ignoring coz deserialization problem//BYTEA
    std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8, //BYTEA
    // type_12: (),//didnt find Encode trait impl in sqlx//BYTEA
    sqlx_postgres_types_pg_interval: SqlxPostgresTypesPgInterval, //INTERVAL
    //INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
    sqlx_postgres_types_pg_range_std_primitive_i64: SqlxPostgresTypesPgRangeStdPrimitiveI64, //INT8RANGE
    sqlx_postgres_types_pg_range_std_primitive_i32: SqlxPostgresTypesPgRangeStdPrimitiveI32, //INT4RANGE
    // type_16: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc, //TSRANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime, //maybe not correct//TSRANGE
    // type_17: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset, //TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,       //TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime, //maybe not correct//TSTZRANGE
    // type_18: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//DATERANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate, //maybe not correct//DATERANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_date: SqlxPostgresTypesPgRangeSqlxTypesTimeDate,        //maybe not correct//DATERANGE
    // type_19: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//NUMRANGE
    sqlx_postgres_types_pg_range_sqlx_types_big_decimal: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal, //NUMRANGE
    sqlx_postgres_types_pg_range_sqlx_types_decimal: SqlxPostgresTypesPgRangeSqlxTypesDecimal,    //NUMRANGE
    sqlx_postgres_types_pg_money: SqlxPostgresTypesPgMoney,                     //MONEY
    sqlx_postgres_types_pg_l_tree: SqlxPostgresTypesPgLTree,                     //LTREE
    sqlx_postgres_types_pg_l_query: SqlxPostgresTypesPgLQuery,                    //LQUERY
    sqlx_postgres_types_pg_ci_text: SqlxPostgresTypesPgCiText,                    //CITEXT
    sqlx_types_big_decimal: SqlxTypesBigDecimal,                          //NUMERIC
    sqlx_types_decimal: SqlxTypesDecimal,                             //NUMERIC
    sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset: SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset, //TIMESTAMPTZ
    sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxTypesChronoDateTimeSqlxTypesChronoLocal,  //TIMESTAMPTZ
    sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxTypesChronoDateTimeSqlxTypesChronoUtc,    //TIMESTAMP
    sqlx_types_chrono_naive_date_time: SqlxTypesChronoNaiveDateTime,//TIMESTAMP
    sqlx_types_chrono_naive_date: SqlxTypesChronoNaiveDate,                     //DATE
    sqlx_types_chrono_naive_time: SqlxTypesChronoNaiveTime,                     //TIME
    sqlx_postgres_types_pg_time_tz: SqlxPostgresTypesPgTimeTz,                    //just present chrono or time flag
    // type_: sqlx::postgres::types::PgTimeTz,//feature flag chrono//TIMETZ
    sqlx_types_time_primitive_date_time: SqlxTypesTimePrimitiveDateTime, //TIMESTAMP
    sqlx_types_time_offset_date_time: SqlxTypesTimeOffsetDateTime,    //TIMESTAMPTZ
    sqlx_types_time_date: SqlxTypesTimeDate,              //DATE
    sqlx_types_time_time: SqlxTypesTimeTime,              //TIME
    // type_: sqlx::postgres::types::PgTimeTz,//feature flag time//TIMETZ
    sqlx_types_uuid_uuid: SqlxTypesUuidUuid,             //UUID
    sqlx_types_ipnetwork_ip_network: SqlxTypesIpnetworkIpNetwork,   //INET, CIDR
    std_net_ip_addr: StdNetIpAddr,                  //INET, CIDR
    sqlx_types_mac_address_mac_address: SqlxTypesMacAddressMacAddress, //MACADDR
    sqlx_types_bit_vec: SqlxTypesBitVec,               //BIT, VARBIT
    sqlx_types_json: SqlxTypesJson<T>,              //JSON, JSONB
    serde_json_value: SerdeJsonValue,                //JSON, JSONB
                                            // type_44: serde_json::value::RawValue,//lifetime and borrow problem//JSON, JSONB
                                            //maybe Composite types
                                            //maybe Enumerations
}

//
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TestNewTypeWithSerializeDeserialize {//<T>
    //https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
    std_primitive_bool: StdPrimitiveBool, //BOOL
    std_primitive_i8: StdPrimitiveI8,   //“CHAR”
    std_primitive_i16: StdPrimitiveI16,  //SMALLINT, SMALLSERIAL, INT2
    std_primitive_i32: StdPrimitiveI32,  //INT, SERIAL, INT4
    std_primitive_i64: StdPrimitiveI64,  //BIGINT, BIGSERIAL, INT8
    std_primitive_f32: StdPrimitiveF32,  //REAL, FLOAT4
    std_primitive_f64: StdPrimitiveF64,  //DOUBLE PRECISION, FLOAT8
    // type_8: &std::primitive::str,//lifetimes are unexpectable i think //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    std_string_string: StdStringString, //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    // type_10: [std::primitive::u8;1],//ignoring coz deserialization problem//BYTEA
    std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8, //BYTEA
    // type_12: (),//didnt find Encode trait impl in sqlx//BYTEA
    sqlx_postgres_types_pg_interval: SqlxPostgresTypesPgIntervalWithSerializeDeserialize, //INTERVAL
    //INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
    // sqlx_postgres_types_pg_range_std_primitive_i64: SqlxPostgresTypesPgRangeStdPrimitiveI64, //INT8RANGE
    // sqlx_postgres_types_pg_range_std_primitive_i32: SqlxPostgresTypesPgRangeStdPrimitiveI32, //INT4RANGE
    // // type_16: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSRANGE
    // sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc, //TSRANGE
    // sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime, //maybe not correct//TSRANGE
    // // type_17: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSTZRANGE
    // sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset, //TSTZRANGE
    // sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,       //TSTZRANGE
    // sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime, //maybe not correct//TSTZRANGE
    // // type_18: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//DATERANGE
    // sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate, //maybe not correct//DATERANGE
    // sqlx_postgres_types_pg_range_sqlx_types_time_date: SqlxPostgresTypesPgRangeSqlxTypesTimeDate,        //maybe not correct//DATERANGE
    // // type_19: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//NUMRANGE
    // sqlx_postgres_types_pg_range_sqlx_types_big_decimal: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal, //NUMRANGE
    // sqlx_postgres_types_pg_range_sqlx_types_decimal: SqlxPostgresTypesPgRangeSqlxTypesDecimal,    //NUMRANGE
    // sqlx_postgres_types_pg_money: SqlxPostgresTypesPgMoney,                     //MONEY
    // sqlx_postgres_types_pg_l_tree: SqlxPostgresTypesPgLTree,                     //LTREE
    // sqlx_postgres_types_pg_l_query: SqlxPostgresTypesPgLQuery,                    //LQUERY
    // sqlx_postgres_types_pg_ci_text: SqlxPostgresTypesPgCiText,                    //CITEXT
    // sqlx_types_big_decimal: SqlxTypesBigDecimal,                          //NUMERIC
    // sqlx_types_decimal: SqlxTypesDecimal,                             //NUMERIC
    // sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset: SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset, //TIMESTAMPTZ
    // sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxTypesChronoDateTimeSqlxTypesChronoLocal,  //TIMESTAMPTZ
    // sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxTypesChronoDateTimeSqlxTypesChronoUtc,    //TIMESTAMP
    // sqlx_types_chrono_naive_date_time: SqlxTypesChronoNaiveDateTime,//TIMESTAMP
    // sqlx_types_chrono_naive_date: SqlxTypesChronoNaiveDate,                     //DATE
    // sqlx_types_chrono_naive_time: SqlxTypesChronoNaiveTime,                     //TIME
    // sqlx_postgres_types_pg_time_tz: SqlxPostgresTypesPgTimeTz,                    //just present chrono or time flag
    // // type_: sqlx::postgres::types::PgTimeTz,//feature flag chrono//TIMETZ
    // sqlx_types_time_primitive_date_time: SqlxTypesTimePrimitiveDateTime, //TIMESTAMP
    // sqlx_types_time_offset_date_time: SqlxTypesTimeOffsetDateTime,    //TIMESTAMPTZ
    // sqlx_types_time_date: SqlxTypesTimeDate,              //DATE
    // sqlx_types_time_time: SqlxTypesTimeTime,              //TIME
    // // type_: sqlx::postgres::types::PgTimeTz,//feature flag time//TIMETZ
    // sqlx_types_uuid_uuid: SqlxTypesUuidUuid,             //UUID
    // sqlx_types_ipnetwork_ip_network: SqlxTypesIpnetworkIpNetwork,   //INET, CIDR
    // std_net_ip_addr: StdNetIpAddr,                  //INET, CIDR
    // sqlx_types_mac_address_mac_address: SqlxTypesMacAddressMacAddress, //MACADDR
    // sqlx_types_bit_vec: SqlxTypesBitVec,               //BIT, VARBIT
    // sqlx_types_json: SqlxTypesJson<T>,              //JSON, JSONB
    // serde_json_value: SerdeJsonValue,                //JSON, JSONB
    // // type_44: serde_json::value::RawValue,//lifetime and borrow problem//JSON, JSONB
    // //maybe Composite types
    // //maybe Enumerations
}
//

impl<T> std::convert::From<Test<T>> for TestNewType<T> {
    fn from(value: Test<T>) -> Self {
        Self {
            std_primitive_bool: StdPrimitiveBool(value.std_primitive_bool), //BOOL
            std_primitive_i8: StdPrimitiveI8(value.std_primitive_i8),   //“CHAR”
            std_primitive_i16: StdPrimitiveI16(value.std_primitive_i16),  //SMALLINT, SMALLSERIAL, INT2
            std_primitive_i32: StdPrimitiveI32(value.std_primitive_i32),  //INT, SERIAL, INT4
            std_primitive_i64: StdPrimitiveI64(value.std_primitive_i64),  //BIGINT, BIGSERIAL, INT8
            std_primitive_f32: StdPrimitiveF32(value.std_primitive_f32),  //REAL, FLOAT4
            std_primitive_f64: StdPrimitiveF64(value.std_primitive_f64),  //DOUBLE PRECISION, FLOAT8
            std_string_string: StdStringString(value.std_string_string), //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
            std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8(value.std_vec_vec_std_primitive_u8), //BYTEA
            sqlx_postgres_types_pg_interval: SqlxPostgresTypesPgInterval(value.sqlx_postgres_types_pg_interval), //INTERVAL
            sqlx_postgres_types_pg_range_std_primitive_i64: SqlxPostgresTypesPgRangeStdPrimitiveI64(value.sqlx_postgres_types_pg_range_std_primitive_i64), //INT8RANGE
            sqlx_postgres_types_pg_range_std_primitive_i32: SqlxPostgresTypesPgRangeStdPrimitiveI32(value.sqlx_postgres_types_pg_range_std_primitive_i32), //INT4RANGE
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
                value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc
            ), //TSRANGE
            sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time
            ), //maybe not correct//TSRANGE
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(
                value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset
            ), //TSTZRANGE
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
                value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local
            ), //TSTZRANGE
            sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time
            ), //maybe not correct//TSTZRANGE
            sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(
                value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date
            ), //maybe not correct//DATERANGE
            sqlx_postgres_types_pg_range_sqlx_types_time_date: SqlxPostgresTypesPgRangeSqlxTypesTimeDate(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_date
            ), //maybe not correct//DATERANGE
            sqlx_postgres_types_pg_range_sqlx_types_big_decimal: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(
                value.sqlx_postgres_types_pg_range_sqlx_types_big_decimal
            ), //NUMRANGE
            sqlx_postgres_types_pg_range_sqlx_types_decimal: SqlxPostgresTypesPgRangeSqlxTypesDecimal(
                value.sqlx_postgres_types_pg_range_sqlx_types_decimal
            ),    //NUMRANGE
            sqlx_postgres_types_pg_money: SqlxPostgresTypesPgMoney(value.sqlx_postgres_types_pg_money),                           //MONEY
            sqlx_postgres_types_pg_l_tree: SqlxPostgresTypesPgLTree(value.sqlx_postgres_types_pg_l_tree),                           //LTREE
            sqlx_postgres_types_pg_l_query: SqlxPostgresTypesPgLQuery(value.sqlx_postgres_types_pg_l_query),                          //LQUERY
            sqlx_postgres_types_pg_ci_text: SqlxPostgresTypesPgCiText(value.sqlx_postgres_types_pg_ci_text),                          //CITEXT
            sqlx_types_big_decimal: SqlxTypesBigDecimal(value.sqlx_types_big_decimal),                                  //NUMERIC
            sqlx_types_decimal: SqlxTypesDecimal(value.sqlx_types_decimal),                                     //NUMERIC
            sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset: SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(
                value.sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset
            ), //TIMESTAMPTZ
            sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxTypesChronoDateTimeSqlxTypesChronoLocal(
                value.sqlx_types_chrono_date_time_sqlx_types_chrono_local
            ), //TIMESTAMPTZ
            sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxTypesChronoDateTimeSqlxTypesChronoUtc(
                value.sqlx_types_chrono_date_time_sqlx_types_chrono_utc
            ),  //TIMESTAMP
            sqlx_types_chrono_naive_date_time: SqlxTypesChronoNaiveDateTime(value.sqlx_types_chrono_naive_date_time),//TIMESTAMP
            sqlx_types_chrono_naive_date: SqlxTypesChronoNaiveDate(value.sqlx_types_chrono_naive_date),                           //DATE
            sqlx_types_chrono_naive_time: SqlxTypesChronoNaiveTime(value.sqlx_types_chrono_naive_time),                           //TIME
            sqlx_postgres_types_pg_time_tz: SqlxPostgresTypesPgTimeTz(value.sqlx_postgres_types_pg_time_tz), //just present chrono or time flag
            sqlx_types_time_primitive_date_time: SqlxTypesTimePrimitiveDateTime(value.sqlx_types_time_primitive_date_time), //TIMESTAMP
            sqlx_types_time_offset_date_time: SqlxTypesTimeOffsetDateTime(value.sqlx_types_time_offset_date_time),    //TIMESTAMPTZ
            sqlx_types_time_date: SqlxTypesTimeDate(value.sqlx_types_time_date),              //DATE
            sqlx_types_time_time: SqlxTypesTimeTime(value.sqlx_types_time_time),              //TIME
            sqlx_types_uuid_uuid: SqlxTypesUuidUuid(value.sqlx_types_uuid_uuid),              //UUID
            sqlx_types_ipnetwork_ip_network: SqlxTypesIpnetworkIpNetwork(value.sqlx_types_ipnetwork_ip_network),    //INET, CIDR
            std_net_ip_addr: StdNetIpAddr(value.std_net_ip_addr),                     //INET, CIDR
            sqlx_types_mac_address_mac_address: SqlxTypesMacAddressMacAddress(value.sqlx_types_mac_address_mac_address), //MACADDR
            sqlx_types_bit_vec: SqlxTypesBitVec(value.sqlx_types_bit_vec),                  //BIT, VARBIT
            sqlx_types_json: SqlxTypesJson::<T>(value.sqlx_types_json),                 //JSON, JSONB
            serde_json_value: SerdeJsonValue(value.serde_json_value),                    //JSON, JSONB
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Something {
    something: std::string::String,
}
//
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TimeMonth {
    January,// = 1,
    February,// = 2,
    March,// = 3,
    April,// = 4,
    May,// = 5,
    June,// = 6,
    July,// = 7,
    August,// = 8,
    September,// = 9,
    October,// = 10,
    November,// = 11,
    December,// = 12,
}
impl std::convert::From<TimeMonth> for time::Month {
    fn from(value: TimeMonth) -> Self {
        match value {
            TimeMonth::January => time::Month::January,
            TimeMonth::February => time::Month::February,
            TimeMonth::March => time::Month::March,
            TimeMonth::April => time::Month::April,
            TimeMonth::May => time::Month::May,
            TimeMonth::June => time::Month::June,
            TimeMonth::July => time::Month::July,
            TimeMonth::August => time::Month::August,
            TimeMonth::September => time::Month::September,
            TimeMonth::October => time::Month::October,
            TimeMonth::November => time::Month::November,
            TimeMonth::December => time::Month::December,
        }
    }
}
impl std::convert::From<time::Month> for TimeMonth {
    fn from(value: time::Month) -> Self {
        match value {
            time::Month::January => Self::January,
            time::Month::February => Self::February,
            time::Month::March => Self::March,
            time::Month::April => Self::April,
            time::Month::May => Self::May,
            time::Month::June => Self::June,
            time::Month::July => Self::July,
            time::Month::August => Self::August,
            time::Month::September => Self::September,
            time::Month::October => Self::October,
            time::Month::November => Self::November,
            time::Month::December => Self::December,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize {
    year: std::primitive::i32,
    month: TimeMonth,
    day: std::primitive::u8
}
//
impl Default for TestNewType<Something> {
    fn default() -> Self {
        let std_primitive_u8_handle = std::primitive::u8::default();
        let std_primitive_i8_handle = std::primitive::i8::default();
        let std_primitive_u16_handle = std::primitive::u16::default();
        let std_primitive_u32_handle = std::primitive::u32::default();
        let std_primitive_i32_handle = std::primitive::i32::default();
        let std_primitive_i64_handle = std::primitive::i64::default();
        let std_string_string_handle = std::string::String::default();
        let sqlx_types_time_date_handle = sqlx::types::time::Date::from_calendar_date(
            2024,
            time::Month::February,
            3,
        )
        .unwrap();
        let sqlx_types_time_time_handle = sqlx::types::time::Time::from_hms(1,1,1).unwrap();
        let sqlx_types_chrono_naive_date_handle = sqlx::types::chrono::NaiveDate::from_ymd_opt(2016, 11, 3).unwrap();
        let sqlx_types_chrono_naive_time_handle = sqlx::types::chrono::NaiveTime::from_hms_opt(10, 10, 10).unwrap();
        let sqlx_types_chrono_naive_date_time_handle = sqlx::types::chrono::NaiveDateTime::new(
            sqlx_types_chrono_naive_date_handle.clone(),//todo
            sqlx_types_chrono_naive_time_handle.clone(),
        );
        let sqlx_types_time_primitive_date_time_handle = sqlx::types::time::PrimitiveDateTime::new(
            sqlx_types_time_date_handle.clone(), //todo
            sqlx_types_time_time_handle.clone(), //todo
        );
        let sqlx_types_chrono_fixed_offset_handle = sqlx::types::chrono::FixedOffset::west_opt(std_primitive_i32_handle.clone()).unwrap();
        let sqlx_types_time_offset_date_time_handle = sqlx::types::time::OffsetDateTime::now_utc();
        let sqlx_types_decimal_handle = sqlx::types::Decimal::try_new(
            std_primitive_i64_handle.clone(),
            std_primitive_u32_handle.clone(),
        )
        .unwrap();
        let sqlx_types_chrono_utc_handle = sqlx::types::chrono::Utc;
        let sqlx_types_big_decimal_handle = sqlx::types::BigDecimal::new(
            num_bigint::BigInt::new(
                num_bigint::Sign::Plus,
                vec![std_primitive_u32_handle.clone()],
            ),
            std_primitive_i64_handle.clone(),
        );
        //
        let std_primitive_bool = StdPrimitiveBool(true);
        let std_primitive_i8 = StdPrimitiveI8(std_primitive_i8_handle.clone());
        let std_primitive_i16 = StdPrimitiveI16(std::primitive::i16::default());
        let std_primitive_i32 = StdPrimitiveI32(std_primitive_i32_handle.clone());
        let std_primitive_i64 = StdPrimitiveI64(std_primitive_i64_handle.clone());
        let std_primitive_f32 = StdPrimitiveF32(std::primitive::f32::default());
        let std_primitive_f64 = StdPrimitiveF64(std::primitive::f64::default());
        let std_string_string = StdStringString(std_string_string_handle.clone());
        let std_vec_vec_std_primitive_u8 = StdVecVecStdPrimitiveU8(vec![std_primitive_u8_handle.clone()]);
        let sqlx_postgres_types_pg_interval =
            SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval {
                months: std_primitive_i32_handle.clone(),
                days: std_primitive_i32_handle.clone(),
                microseconds: std_primitive_i64_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_std_primitive_i64 =
            SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange::<
                std::primitive::i64,
            > {
                start: std::ops::Bound::Included(std_primitive_i64_handle.clone()),
                end: std::ops::Bound::Included(std_primitive_i64_handle.clone()),
            });
        let sqlx_postgres_types_pg_range_std_primitive_i32 =
            SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange::<
                std::primitive::i32,
            > {
                start: std::ops::Bound::Included(std_primitive_i32_handle.clone()),
                end: std::ops::Bound::Included(std_primitive_i32_handle.clone()),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
            sqlx::postgres::types::PgRange::<
                sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
            > {
                start: std::ops::Bound::Included(sqlx::types::chrono::DateTime::from_naive_utc_and_offset(
                    sqlx_types_chrono_naive_date_time_handle.clone(),
                    sqlx_types_chrono_utc_handle.clone()
                )),
                end: std::ops::Bound::Included(sqlx::types::chrono::DateTime::from_naive_utc_and_offset(
                    sqlx_types_chrono_naive_date_time_handle.clone(),
                    sqlx_types_chrono_utc_handle.clone()
                )),
            }
        );
        let sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time =
            SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(sqlx::postgres::types::PgRange::<
                sqlx::types::time::PrimitiveDateTime,
            > {
                start: std::ops::Bound::Included(
                    sqlx_types_time_primitive_date_time_handle.clone(),
                ),
                end: std::ops::Bound::Included(
                    sqlx_types_time_primitive_date_time_handle.clone(),
                ),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset =
            SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(
                sqlx::postgres::types::PgRange::<
                    sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
                > {
                    start: std::ops::Bound::Included(sqlx::types::chrono::DateTime::from_naive_utc_and_offset(
                        sqlx_types_chrono_naive_date_time_handle.clone(),
                        sqlx_types_chrono_fixed_offset_handle.clone(),
                    )),
                    end: std::ops::Bound::Included(sqlx::types::chrono::DateTime::from_naive_utc_and_offset(
                        sqlx_types_chrono_naive_date_time_handle.clone(),
                        sqlx_types_chrono_fixed_offset_handle.clone(),
                    )),
                },
            );
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
            sqlx::postgres::types::PgRange::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> {
                start: std::ops::Bound::Included(sqlx::types::chrono::DateTime::from_naive_utc_and_offset(
                    sqlx_types_chrono_naive_date_time_handle.clone(),
                    sqlx_types_chrono_fixed_offset_handle.clone()
                )),
                end: std::ops::Bound::Included(sqlx::types::chrono::DateTime::from_naive_utc_and_offset(
                    sqlx_types_chrono_naive_date_time_handle.clone(),
                    sqlx_types_chrono_fixed_offset_handle.clone()
                )),
            }
        );
        let sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time =
            SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(sqlx::postgres::types::PgRange::<
                sqlx::types::time::OffsetDateTime,
            > {
                start: std::ops::Bound::Included(
                    sqlx_types_time_offset_date_time_handle.clone(),
                ),
                end: std::ops::Bound::Included(
                    sqlx_types_time_offset_date_time_handle.clone(),
                ),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date =
            SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(sqlx::postgres::types::PgRange::<
                sqlx::types::chrono::NaiveDate,
            > {
                start: std::ops::Bound::Included(
                    sqlx_types_chrono_naive_date_handle.clone(),
                ),
                end: std::ops::Bound::Included(
                    sqlx_types_chrono_naive_date_handle.clone(),
                ),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_time_date =
            SqlxPostgresTypesPgRangeSqlxTypesTimeDate(sqlx::postgres::types::PgRange::<
                sqlx::types::time::Date,
            > {
                start: std::ops::Bound::Included(
                    sqlx_types_time_date_handle.clone(),
                ),
                end: std::ops::Bound::Included(
                    sqlx_types_time_date_handle.clone(),
                ),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_big_decimal =
            SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(sqlx::postgres::types::PgRange::<
                sqlx::types::BigDecimal,
            > {
                start: std::ops::Bound::Included(sqlx_types_big_decimal_handle.clone()),
                end: std::ops::Bound::Included(sqlx_types_big_decimal_handle.clone()),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_decimal =
            SqlxPostgresTypesPgRangeSqlxTypesDecimal(sqlx::postgres::types::PgRange::<
                sqlx::types::Decimal,
            > {
                start: std::ops::Bound::Included(
                    sqlx_types_decimal_handle.clone(),
                ),
                end: std::ops::Bound::Included(
                    sqlx_types_decimal_handle.clone(),
                ),
            });
        let sqlx_postgres_types_pg_money = SqlxPostgresTypesPgMoney(sqlx::postgres::types::PgMoney(
            std_primitive_i64_handle.clone(),
        ));
        let sqlx_postgres_types_pg_l_tree =
            SqlxPostgresTypesPgLTree(sqlx::postgres::types::PgLTree::new());
        let sqlx_postgres_types_pg_l_query =
            SqlxPostgresTypesPgLQuery(sqlx::postgres::types::PgLQuery::new());
        let sqlx_postgres_types_pg_ci_text = SqlxPostgresTypesPgCiText(
            sqlx::postgres::types::PgCiText(std_string_string_handle.clone()),
        );
        let sqlx_types_big_decimal = SqlxTypesBigDecimal(sqlx_types_big_decimal_handle.clone());
        let sqlx_types_decimal = SqlxTypesDecimal(
            sqlx_types_decimal_handle.clone(),
        );
        let sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset =
            SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(sqlx::types::chrono::DateTime::<
                sqlx::types::chrono::FixedOffset,
            >::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle.clone(),
                sqlx_types_chrono_fixed_offset_handle,
            ));
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local =
            SqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::types::chrono::DateTime::<
                sqlx::types::chrono::Local,
            >::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle.clone(),
                sqlx_types_chrono_fixed_offset_handle,
            ));
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc =
            SqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::types::chrono::DateTime::<
                sqlx::types::chrono::Utc,
            >::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle.clone(),
                sqlx_types_chrono_utc_handle.clone()
            ));
        let sqlx_types_chrono_naive_date_time = SqlxTypesChronoNaiveDateTime(
            sqlx_types_chrono_naive_date_time_handle.clone()
        );
        let sqlx_types_chrono_naive_date = SqlxTypesChronoNaiveDate(
            sqlx_types_chrono_naive_date_handle.clone(),
        );
        let sqlx_types_chrono_naive_time = SqlxTypesChronoNaiveTime(sqlx_types_chrono_naive_time_handle.clone());
        let sqlx_postgres_types_pg_time_tz =
            SqlxPostgresTypesPgTimeTz(sqlx::postgres::types::PgTimeTz {
                time: sqlx_types_time_time_handle.clone(),
                offset: sqlx::types::time::UtcOffset::from_hms(
                    std_primitive_i8_handle.clone(),
                    std_primitive_i8_handle.clone(),
                    std_primitive_i8_handle.clone(),
                )
                .unwrap(),
            });
        let sqlx_types_time_primitive_date_time =
            SqlxTypesTimePrimitiveDateTime(sqlx_types_time_primitive_date_time_handle.clone());
        let sqlx_types_time_offset_date_time =
            SqlxTypesTimeOffsetDateTime(sqlx_types_time_offset_date_time_handle.clone());
        let sqlx_types_time_date = SqlxTypesTimeDate(
            sqlx_types_time_date_handle.clone(),
        );
        let sqlx_types_time_time = SqlxTypesTimeTime(
            sqlx_types_time_time_handle.clone(),
        );
        let sqlx_types_uuid_uuid = SqlxTypesUuidUuid(sqlx::types::uuid::Uuid::from_u128(
            std::primitive::u128::default(),
        ));
        let sqlx_types_ipnetwork_ip_network =
            SqlxTypesIpnetworkIpNetwork(sqlx::types::ipnetwork::IpNetwork::V6(
                sqlx::types::ipnetwork::Ipv6Network::new(
                    std::net::Ipv6Addr::new(
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                    ),
                    std_primitive_u8_handle.clone(),
                )
                .unwrap(),
            ));
        let std_net_ip_addr = StdNetIpAddr(std::net::IpAddr::V6(core::net::Ipv6Addr::new(
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
        )));
        let sqlx_types_mac_address_mac_address =
            SqlxTypesMacAddressMacAddress(sqlx::types::mac_address::MacAddress::new([
                std_primitive_u8_handle.clone(),
                std_primitive_u8_handle.clone(),
                std_primitive_u8_handle.clone(),
                std_primitive_u8_handle.clone(),
                std_primitive_u8_handle.clone(),
                std_primitive_u8_handle.clone(),
            ]));
        let sqlx_types_bit_vec = SqlxTypesBitVec(sqlx::types::BitVec::new());
        let sqlx_types_json = SqlxTypesJson(sqlx::types::Json(Something {
            something: std_string_string_handle.clone(),
        }));
        let serde_json_value = SerdeJsonValue(serde_json::Value::Bool(std::primitive::bool::default()));
        //
        Self {
            std_primitive_bool,
            std_primitive_i8,
            std_primitive_i16,
            std_primitive_i32,
            std_primitive_i64,
            std_primitive_f32,
            std_primitive_f64,
            std_string_string,
            std_vec_vec_std_primitive_u8,
            sqlx_postgres_types_pg_interval,
            sqlx_postgres_types_pg_range_std_primitive_i64,
            sqlx_postgres_types_pg_range_std_primitive_i32,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
            sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local,
            sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date,
            sqlx_postgres_types_pg_range_sqlx_types_time_date,
            sqlx_postgres_types_pg_range_sqlx_types_big_decimal,
            sqlx_postgres_types_pg_range_sqlx_types_decimal,
            sqlx_postgres_types_pg_money,
            sqlx_postgres_types_pg_l_tree,
            sqlx_postgres_types_pg_l_query,
            sqlx_postgres_types_pg_ci_text,
            sqlx_types_big_decimal,
            sqlx_types_decimal,
            sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset,
            sqlx_types_chrono_date_time_sqlx_types_chrono_local,
            sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
            sqlx_types_chrono_naive_date_time,
            sqlx_types_chrono_naive_date,
            sqlx_types_chrono_naive_time,
            sqlx_postgres_types_pg_time_tz,
            sqlx_types_time_primitive_date_time,
            sqlx_types_time_offset_date_time,
            sqlx_types_time_date,
            sqlx_types_time_time,
            sqlx_types_uuid_uuid,
            sqlx_types_ipnetwork_ip_network,
            std_net_ip_addr,
            sqlx_types_mac_address_mac_address,
            sqlx_types_bit_vec,
            sqlx_types_json,
            serde_json_value,
        }
    }
}

pub trait IntoSerdeSerializeDeserialize {}

pub trait PostgresqlFilter {}

// impl PostgresqlFilter for sqlx::types:: {}

pub trait PostgresqlOrder {}

pub trait PostgresqlLimit {}

// integer, bigint
// real, double precision
// varchar
// text
// jsonb
// tsvector
// int4range
// daterange

// impl trait PostgresqlLimit for sqlx::types:: {}

//todo swagger type\schema

pub trait PostgersqlColumn<'a>:
    std::fmt::Debug
    + IntoSerdeSerializeDeserialize
    + utoipa::ToSchema<'a>
    + PostgresqlFilter
    + PostgresqlOrder
    + PostgresqlLimit
{
}

pub trait PostgresqlSerdeSerialize<T: serde::Serialize> {
    fn serde_serialize() -> T;
}

pub trait CheckSupportedPostgresqlColumnType {
    fn check_supported_postgresql_column_type();
}
//new type pattern
// sqlx::Encode impl was copied from https://docs.rs/sqlx/0.7.3/sqlx/trait.Encode.html
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveBool(pub std::primitive::bool);
impl StdPrimitiveBool {
    pub fn into_inner(self) -> std::primitive::bool {
        self.0
    }
}
impl std::convert::From<StdPrimitiveBool> for std::primitive::bool {
    fn from(value: StdPrimitiveBool) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBool {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveBool {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBool {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveBool {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlBool for StdPrimitiveBool {}
impl PostgresqlOrder for StdPrimitiveBool {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI8(pub std::primitive::i8);
impl StdPrimitiveI8 {
    pub fn into_inner(self) -> std::primitive::i8 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI8> for std::primitive::i8 {
    fn from(value: StdPrimitiveI8) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI8 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i8 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i8 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI8 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI8 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI8 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlChar for StdPrimitiveI8 {}
impl PostgresqlOrder for StdPrimitiveI8 {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI16(pub std::primitive::i16);
impl StdPrimitiveI16 {
    pub fn into_inner(self) -> std::primitive::i16 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI16> for std::primitive::i16 {
    fn from(value: StdPrimitiveI16) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI16 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i16 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i16 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI16 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI16 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI16 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlSmallInt for StdPrimitiveI16 {}
impl AsPostgresqlSmallSerial for StdPrimitiveI16 {}
impl AsPostgresqlInt2 for StdPrimitiveI16 {}
impl PostgresqlOrder for StdPrimitiveI16 {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI32(pub std::primitive::i32);
impl StdPrimitiveI32 {
    pub fn into_inner(self) -> std::primitive::i32 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI32> for std::primitive::i32 {
    fn from(value: StdPrimitiveI32) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI32 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI32 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI32 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI32 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlReal for StdPrimitiveI32 {}
impl AsPostgresqlFloat4 for StdPrimitiveI32 {}
impl PostgresqlOrder for StdPrimitiveI32 {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI64(pub std::primitive::i64);
impl StdPrimitiveI64 {
    pub fn into_inner(self) -> std::primitive::i64 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI64> for std::primitive::i64 {
    fn from(value: StdPrimitiveI64) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI64 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI64 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlBigInt for StdPrimitiveI64 {}
impl AsPostgresqlBigSerial for StdPrimitiveI64 {}
impl AsPostgresqlInt8 for StdPrimitiveI64 {}
impl PostgresqlOrder for StdPrimitiveI64 {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveF32(pub std::primitive::f32);
impl StdPrimitiveF32 {
    pub fn into_inner(self) -> std::primitive::f32 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveF32> for std::primitive::f32 {
    fn from(value: StdPrimitiveF32) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveF32 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::f32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::f32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveF32 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveF32 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveF32 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlReal for StdPrimitiveF32 {}
impl AsPostgresqlFloat4 for StdPrimitiveF32 {}
impl PostgresqlOrder for StdPrimitiveF32 {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveF64(pub std::primitive::f64);
impl StdPrimitiveF64 {
    pub fn into_inner(self) -> std::primitive::f64 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveF64> for std::primitive::f64 {
    fn from(value: StdPrimitiveF64) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveF64 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::f64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::f64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveF64 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveF64 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveF64 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlDoublePrecision for StdPrimitiveF64 {}
impl AsPostgresqlFloat8 for StdPrimitiveF64 {}
impl PostgresqlOrder for StdPrimitiveF64 {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdStringString(pub std::string::String);
impl StdStringString {
    pub fn into_inner(self) -> std::string::String {
        self.0
    }
}
impl std::convert::From<StdStringString> for std::string::String {
    fn from(value: StdStringString) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdStringString {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::string::String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::string::String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdStringString {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdStringString {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdStringString {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlVarchar for StdStringString {}
impl AsPostgresqlCharN for StdStringString {}
impl AsPostgresqlText for StdStringString {}
impl AsPostgresqlName for StdStringString {}
impl AsPostgresqlCiText for StdStringString {}
impl PostgresqlOrder for StdStringString {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
impl StdVecVecStdPrimitiveU8 {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::u8> {
        self.0
    }
}
impl std::convert::From<StdVecVecStdPrimitiveU8> for std::vec::Vec<std::primitive::u8> {
    fn from(value: StdVecVecStdPrimitiveU8) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdVecVecStdPrimitiveU8 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::vec::Vec<std::primitive::u8> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::vec::Vec<std::primitive::u8> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdVecVecStdPrimitiveU8 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdVecVecStdPrimitiveU8 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdVecVecStdPrimitiveU8 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlBytea for StdVecVecStdPrimitiveU8 {}
pub struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxPostgresTypesPgIntervalWithSerializeDeserialize {
    pub months: std::primitive::i32,
    pub days: std::primitive::i32,
    pub microseconds: std::primitive::i64,
}
impl std::convert::From<SqlxPostgresTypesPgIntervalWithSerializeDeserialize> for sqlx::postgres::types::PgInterval {
    fn from(value: SqlxPostgresTypesPgIntervalWithSerializeDeserialize) -> Self {
        Self {
            months: value.months,
            days: value.days,
            microseconds: value.microseconds,
        }
    }
}
impl std::convert::From<sqlx::postgres::types::PgInterval> for SqlxPostgresTypesPgIntervalWithSerializeDeserialize {
    fn from(value: sqlx::postgres::types::PgInterval) -> Self {
        Self {
            months: value.months,
            days: value.days,
            microseconds: value.microseconds,
        }
    }
}
impl SqlxPostgresTypesPgInterval {
    pub fn into_inner(self) -> sqlx::postgres::types::PgInterval {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgInterval> for sqlx::postgres::types::PgInterval {
    fn from(value: SqlxPostgresTypesPgInterval) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgInterval {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgInterval {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgInterval {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgInterval {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInterval for SqlxPostgresTypesPgInterval {}
impl PostgresqlOrder for SqlxPostgresTypesPgInterval {}
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(
    pub sqlx::postgres::types::PgRange<std::primitive::i64>,
);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize {
    pub start: std::ops::Bound<std::primitive::i64>,
    pub end: std::ops::Bound<std::primitive::i64>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize> for sqlx::postgres::types::PgRange<std::primitive::i64> {
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}
impl std::convert::From<sqlx::postgres::types::PgRange<std::primitive::i64>> for SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize {
    fn from(value: sqlx::postgres::types::PgRange<std::primitive::i64>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}
impl SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<std::primitive::i64> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI64>
    for sqlx::postgres::types::PgRange<std::primitive::i64>
{
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI64) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<std::primitive::i64> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<std::primitive::i64> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInt8Range for SqlxPostgresTypesPgRangeStdPrimitiveI64 {}
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(
    pub sqlx::postgres::types::PgRange<std::primitive::i32>,
);
//
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize {
    pub start: std::ops::Bound<std::primitive::i32>,
    pub end: std::ops::Bound<std::primitive::i32>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize> for sqlx::postgres::types::PgRange<std::primitive::i32> {
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}
impl std::convert::From<sqlx::postgres::types::PgRange<std::primitive::i32>> for SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize {
    fn from(value: sqlx::postgres::types::PgRange<std::primitive::i32>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}
//
impl SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<std::primitive::i32> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI32>
    for sqlx::postgres::types::PgRange<std::primitive::i32>
{
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI32) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInt4Range for SqlxPostgresTypesPgRangeStdPrimitiveI32 {}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
);
//
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize {
//     pub start: std::ops::Bound<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
//     pub end: std::ops::Bound<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
// }
// impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize> for sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> {
//     fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize) -> Self {
//         Self {
//             start: value.start,
//             end: value.end,
//         }
//     }
// }
// impl std::convert::From<sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>> for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize {
//     fn from(value: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>) -> Self {
//         Self {
//             start: value.start,
//             end: value.end,
//         }
//     }
// }
//
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    pub fn into_inner(
        self,
    ) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>
    {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc>
    for sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
{
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
{
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsRange for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {}
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    pub fn into_inner(
        self,
    ) -> sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime>
    for sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsRange for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(
    pub  sqlx::postgres::types::PgRange<
        sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
    >,
);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    pub fn into_inner(
        self,
    ) -> sqlx::postgres::types::PgRange<
        sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
    > {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset>
    for sqlx::postgres::types::PgRange<
        sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
    >
{
    fn from(
        value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset,
    ) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset
{
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<
            sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
        > as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<
            sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
        > as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset
{
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsTzRange
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset
{
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    pub fn into_inner(
        self,
    ) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>
    {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal>
    for sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
{
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
{
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {}
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime>
    for sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {}
pub struct SqlxTypesChronoNaiveDateTime(pub sqlx::types::chrono::NaiveDateTime);
impl SqlxTypesChronoNaiveDateTime {
    pub fn into_inner(self) -> sqlx::types::chrono::NaiveDateTime {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoNaiveDateTime>
    for sqlx::types::chrono::NaiveDateTime
{
    fn from(value: SqlxTypesChronoNaiveDateTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveDateTime as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::NaiveDateTime as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoNaiveDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsTzRange for SqlxTypesChronoNaiveDateTime {}
impl PostgresqlOrder for SqlxTypesChronoNaiveDateTime {}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate>
    for sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate> as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlDateRange for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {}
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::Date>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::time::Date> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeDate>
    for sqlx::postgres::types::PgRange<sqlx::types::time::Date>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeDate) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::time::Date> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::time::Date> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlDateRange for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {}
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(
    pub sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesBigDecimal>
    for sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlNumRange for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {}
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(
    pub sqlx::postgres::types::PgRange<sqlx::types::Decimal>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::Decimal> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesDecimal>
    for sqlx::postgres::types::PgRange<sqlx::types::Decimal>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesDecimal) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::Decimal> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::Decimal> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlNumRange for SqlxPostgresTypesPgRangeSqlxTypesDecimal {}
pub struct SqlxPostgresTypesPgMoney(pub sqlx::postgres::types::PgMoney);
impl SqlxPostgresTypesPgMoney {
    pub fn into_inner(self) -> sqlx::postgres::types::PgMoney {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgMoney> for sqlx::postgres::types::PgMoney {
    fn from(value: SqlxPostgresTypesPgMoney) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgMoney {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgMoney as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgMoney as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgMoney {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgMoney {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgMoney {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlMoney for SqlxPostgresTypesPgMoney {}
pub struct SqlxPostgresTypesPgLTree(pub sqlx::postgres::types::PgLTree);
impl SqlxPostgresTypesPgLTree {
    pub fn into_inner(self) -> sqlx::postgres::types::PgLTree {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgLTree> for sqlx::postgres::types::PgLTree {
    fn from(value: SqlxPostgresTypesPgLTree) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgLTree {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgLTree as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgLTree as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgLTree {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgLTree {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgLTree {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlLTree for SqlxPostgresTypesPgLTree {}
pub struct SqlxPostgresTypesPgLQuery(pub sqlx::postgres::types::PgLQuery);
impl SqlxPostgresTypesPgLQuery {
    pub fn into_inner(self) -> sqlx::postgres::types::PgLQuery {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgLQuery> for sqlx::postgres::types::PgLQuery {
    fn from(value: SqlxPostgresTypesPgLQuery) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgLQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgLQuery as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgLQuery as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgLQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgLQuery {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgLQuery {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlLQuery for SqlxPostgresTypesPgLQuery {}
pub struct SqlxPostgresTypesPgCiText(pub sqlx::postgres::types::PgCiText);
impl SqlxPostgresTypesPgCiText {
    pub fn into_inner(self) -> sqlx::postgres::types::PgCiText {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgCiText> for sqlx::postgres::types::PgCiText {
    fn from(value: SqlxPostgresTypesPgCiText) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgCiText {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgCiText as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgCiText as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgCiText {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgCiText {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgCiText {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlCiText for SqlxPostgresTypesPgCiText {}
pub struct SqlxTypesBigDecimal(pub sqlx::types::BigDecimal);
impl SqlxTypesBigDecimal {
    pub fn into_inner(self) -> sqlx::types::BigDecimal {
        self.0
    }
}
impl std::convert::From<SqlxTypesBigDecimal> for sqlx::types::BigDecimal {
    fn from(value: SqlxTypesBigDecimal) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesBigDecimal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::BigDecimal as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::BigDecimal as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesBigDecimal {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesBigDecimal {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesBigDecimal {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlNumeric for SqlxTypesBigDecimal {}
impl PostgresqlOrder for SqlxTypesBigDecimal {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxTypesDecimal(pub sqlx::types::Decimal);
impl SqlxTypesDecimal {
    pub fn into_inner(self) -> sqlx::types::Decimal {
        self.0
    }
}
impl std::convert::From<SqlxTypesDecimal> for sqlx::types::Decimal {
    fn from(value: SqlxTypesDecimal) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesDecimal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Decimal as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Decimal as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesDecimal {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesDecimal {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesDecimal {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlNumeric for SqlxTypesDecimal {}
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
);
impl SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    pub fn into_inner(self) -> sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset> {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset>
    for sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>
{
    fn from(value: SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset> as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimeTz for SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {}
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
);
impl SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    pub fn into_inner(self) -> sqlx::types::chrono::DateTime<sqlx::types::chrono::Local> {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoLocal>
    for sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>
{
    fn from(value: SqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Local> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Local> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimestampTz for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {}
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
);
//
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize {
//     pub start: std::ops::Bound<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
//     pub end: std::ops::Bound<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
// }
// impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize> for sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> {
//     fn from(value: SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize) -> Self {
//         Self {
//             start: value.start,
//             end: value.end,
//         }
//     }
// }
// impl std::convert::From<sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>> for SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize {
//     fn from(value: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>) -> Self {
//         Self {
//             start: value.start,
//             end: value.end,
//         }
//     }
// }
//
impl SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    pub fn into_inner(self) -> sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoUtc>
    for sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>
{
    fn from(value: SqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimestamp for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {}
pub struct SqlxTypesChronoNaiveDate(pub sqlx::types::chrono::NaiveDate);
impl SqlxTypesChronoNaiveDate {
    pub fn into_inner(self) -> sqlx::types::chrono::NaiveDate {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoNaiveDate> for sqlx::types::chrono::NaiveDate {
    fn from(value: SqlxTypesChronoNaiveDate) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveDate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveDate as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::NaiveDate as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoNaiveDate {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlDate for SqlxTypesChronoNaiveDate {}
impl PostgresqlOrder for SqlxTypesChronoNaiveDate {}
pub struct SqlxTypesChronoNaiveTime(pub sqlx::types::chrono::NaiveTime);
impl SqlxTypesChronoNaiveTime {
    pub fn into_inner(self) -> sqlx::types::chrono::NaiveTime {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoNaiveTime> for sqlx::types::chrono::NaiveTime {
    fn from(value: SqlxTypesChronoNaiveTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveTime as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::NaiveTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoNaiveTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTime for SqlxTypesChronoNaiveTime {}
impl PostgresqlOrder for SqlxTypesChronoNaiveTime {}
pub struct SqlxPostgresTypesPgTimeTz(pub sqlx::postgres::types::PgTimeTz);
impl SqlxPostgresTypesPgTimeTz {
    pub fn into_inner(self) -> sqlx::postgres::types::PgTimeTz {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgTimeTz> for sqlx::postgres::types::PgTimeTz {
    fn from(value: SqlxPostgresTypesPgTimeTz) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgTimeTz {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgTimeTz as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgTimeTz as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgTimeTz {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgTimeTz {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgTimeTz {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimeTz for SqlxPostgresTypesPgTimeTz {}
pub struct SqlxTypesTimePrimitiveDateTime(pub sqlx::types::time::PrimitiveDateTime);
impl SqlxTypesTimePrimitiveDateTime {
    pub fn into_inner(self) -> sqlx::types::time::PrimitiveDateTime {
        self.0
    }
}
impl std::convert::From<SqlxTypesTimePrimitiveDateTime> for sqlx::types::time::PrimitiveDateTime {
    fn from(value: SqlxTypesTimePrimitiveDateTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesTimePrimitiveDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::time::PrimitiveDateTime as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::time::PrimitiveDateTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimePrimitiveDateTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimePrimitiveDateTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimePrimitiveDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimestamp for SqlxTypesTimePrimitiveDateTime {}
impl PostgresqlOrder for SqlxTypesTimePrimitiveDateTime {}
pub struct SqlxTypesTimeOffsetDateTime(pub sqlx::types::time::OffsetDateTime);
impl SqlxTypesTimeOffsetDateTime {
    pub fn into_inner(self) -> sqlx::types::time::OffsetDateTime {
        self.0
    }
}
impl std::convert::From<SqlxTypesTimeOffsetDateTime> for sqlx::types::time::OffsetDateTime {
    fn from(value: SqlxTypesTimeOffsetDateTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesTimeOffsetDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::time::OffsetDateTime as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::time::OffsetDateTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimeOffsetDateTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimeOffsetDateTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimeOffsetDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimestampTz for SqlxTypesTimeOffsetDateTime {}
pub struct SqlxTypesTimeDate(pub sqlx::types::time::Date);
impl SqlxTypesTimeDate {
    pub fn into_inner(self) -> sqlx::types::time::Date {
        self.0
    }
}
impl std::convert::From<SqlxTypesTimeDate> for sqlx::types::time::Date {
    fn from(value: SqlxTypesTimeDate) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesTimeDate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::time::Date as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::time::Date as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimeDate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimeDate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimeDate {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlDate for SqlxTypesTimeDate {}
impl PostgresqlOrder for SqlxTypesTimeDate {}
pub struct SqlxTypesTimeTime(pub sqlx::types::time::Time);
impl SqlxTypesTimeTime {
    pub fn into_inner(self) -> sqlx::types::time::Time {
        self.0
    }
}
impl std::convert::From<SqlxTypesTimeTime> for sqlx::types::time::Time {
    fn from(value: SqlxTypesTimeTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesTimeTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::time::Time as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::time::Time as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimeTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimeTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimeTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTime for SqlxTypesTimeTime {}
impl PostgresqlOrder for SqlxTypesTimeTime {}
pub struct SqlxTypesUuidUuid(pub sqlx::types::uuid::Uuid);
impl SqlxTypesUuidUuid {
    pub fn into_inner(self) -> sqlx::types::uuid::Uuid {
        self.0
    }
}
impl std::convert::From<SqlxTypesUuidUuid> for sqlx::types::uuid::Uuid {
    fn from(value: SqlxTypesUuidUuid) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesUuidUuid {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::uuid::Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::uuid::Uuid as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesUuidUuid {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesUuidUuid {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesUuidUuid {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlUuid for SqlxTypesUuidUuid {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxTypesIpnetworkIpNetwork(pub sqlx::types::ipnetwork::IpNetwork);
impl SqlxTypesIpnetworkIpNetwork {
    pub fn into_inner(self) -> sqlx::types::ipnetwork::IpNetwork {
        self.0
    }
}
impl std::convert::From<SqlxTypesIpnetworkIpNetwork> for sqlx::types::ipnetwork::IpNetwork {
    fn from(value: SqlxTypesIpnetworkIpNetwork) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesIpnetworkIpNetwork {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::ipnetwork::IpNetwork as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::ipnetwork::IpNetwork as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesIpnetworkIpNetwork {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesIpnetworkIpNetwork {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesIpnetworkIpNetwork {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInet for SqlxTypesIpnetworkIpNetwork {}
impl AsPostgresqlCidr for SqlxTypesIpnetworkIpNetwork {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdNetIpAddr(pub std::net::IpAddr);
impl StdNetIpAddr {
    pub fn into_inner(self) -> std::net::IpAddr {
        self.0
    }
}
impl std::convert::From<StdNetIpAddr> for std::net::IpAddr {
    fn from(value: StdNetIpAddr) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdNetIpAddr {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::net::IpAddr as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::net::IpAddr as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdNetIpAddr {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdNetIpAddr {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdNetIpAddr {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInet for StdNetIpAddr {}
impl AsPostgresqlCidr for StdNetIpAddr {}
pub struct SqlxTypesMacAddressMacAddress(pub sqlx::types::mac_address::MacAddress);
impl SqlxTypesMacAddressMacAddress {
    pub fn into_inner(self) -> sqlx::types::mac_address::MacAddress {
        self.0
    }
}
impl std::convert::From<SqlxTypesMacAddressMacAddress> for sqlx::types::mac_address::MacAddress {
    fn from(value: SqlxTypesMacAddressMacAddress) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesMacAddressMacAddress {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::mac_address::MacAddress as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::mac_address::MacAddress as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesMacAddressMacAddress {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesMacAddressMacAddress {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesMacAddressMacAddress {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlMacAddr for SqlxTypesMacAddressMacAddress {}
pub struct SqlxTypesBitVec(pub sqlx::types::BitVec);
impl SqlxTypesBitVec {
    pub fn into_inner(self) -> sqlx::types::BitVec {
        self.0
    }
}
impl std::convert::From<SqlxTypesBitVec> for sqlx::types::BitVec {
    fn from(value: SqlxTypesBitVec) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesBitVec {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::BitVec as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::BitVec as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesBitVec {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesBitVec {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesBitVec {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlBit for SqlxTypesBitVec {}
impl AsPostgresqlVarBit for SqlxTypesBitVec {}
impl PostgresqlOrder for SqlxTypesBitVec {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxTypesJson<T>(pub sqlx::types::Json<T>);
impl<T> SqlxTypesJson<T> {
    pub fn into_inner(self) -> sqlx::types::Json<T> {
        self.0
    }
}
impl<T> std::convert::From<SqlxTypesJson<T>> for sqlx::types::Json<T> {
    fn from(value: SqlxTypesJson<T>) -> Self {
        value.0
    }
}
impl<T> sqlx::Type<sqlx::Postgres> for SqlxTypesJson<T> {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<T> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<T> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl<'a, T> sqlx::Encode<'a, sqlx::Postgres> for SqlxTypesJson<T>
where
    T: sqlx::Encode<'a, sqlx::Postgres>
        + Copy
        + Clone
        + std::fmt::Debug
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + std::hash::Hash
        + Default
        + serde::Serialize
        + serde::Deserialize<'a>, //todo maybe add another traits impls
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl<'a, T: 'a> sqlx::Decode<'a, sqlx::Postgres> for SqlxTypesJson<T>
where
    T: sqlx::Decode<'a, sqlx::Postgres>
        + Copy
        + Clone
        + std::fmt::Debug
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + std::hash::Hash
        + Default
        + serde::Serialize
        + serde::Deserialize<'a>, //todo maybe add another traits impls
{
    fn decode(value: sqlx::postgres::PgValueRef<'a>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl<T> CheckSupportedPostgresqlColumnType for SqlxTypesJson<T> {
    fn check_supported_postgresql_column_type() {}
}
impl<T> AsPostgresqlJson for SqlxTypesJson<T> {}
impl<T> AsPostgresqlJsonB for SqlxTypesJson<T> {}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SerdeJsonValue(pub serde_json::Value);
impl SerdeJsonValue {
    pub fn into_inner(self) -> serde_json::Value {
        self.0
    }
}
impl std::convert::From<SerdeJsonValue> for serde_json::Value {
    fn from(value: SerdeJsonValue) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SerdeJsonValue {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <serde_json::Value as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <serde_json::Value as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SerdeJsonValue {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> std::primitive::usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SerdeJsonValue {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SerdeJsonValue {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlJson for SerdeJsonValue {}
impl AsPostgresqlJsonB for SerdeJsonValue {}

pub async fn something() {
    let mut query = sqlx::query::<sqlx::Postgres>("test");
    query = query.bind(Into::<bool>::into(StdPrimitiveBool(false)));
    query = query.bind(StdPrimitiveBool(false).into_inner());
    query = query.bind(StdPrimitiveBool(false));
}

pub fn test_check_supported_postgresql_column_type() {
    //
    //todo check if init functions are not panics. change to not panic init functions

    //
    //
    StdPrimitiveBool::check_supported_postgresql_column_type();
    StdPrimitiveI8::check_supported_postgresql_column_type();
    StdPrimitiveI16::check_supported_postgresql_column_type();
    StdPrimitiveI32::check_supported_postgresql_column_type();
    StdPrimitiveI64::check_supported_postgresql_column_type();
    StdPrimitiveF32::check_supported_postgresql_column_type();
    StdPrimitiveF64::check_supported_postgresql_column_type();
    StdStringString::check_supported_postgresql_column_type();
    StdVecVecStdPrimitiveU8::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgInterval::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeStdPrimitiveI64::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeStdPrimitiveI32::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime::check_supported_postgresql_column_type(
    );
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimeDate::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesBigDecimal::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesDecimal::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgMoney::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgLTree::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgLQuery::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgCiText::check_supported_postgresql_column_type();
    SqlxTypesBigDecimal::check_supported_postgresql_column_type();
    SqlxTypesDecimal::check_supported_postgresql_column_type();
    SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset::check_supported_postgresql_column_type();
    SqlxTypesChronoDateTimeSqlxTypesChronoLocal::check_supported_postgresql_column_type();
    SqlxTypesChronoDateTimeSqlxTypesChronoUtc::check_supported_postgresql_column_type();
    SqlxTypesChronoNaiveDate::check_supported_postgresql_column_type();
    SqlxTypesChronoNaiveTime::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgTimeTz::check_supported_postgresql_column_type();
    SqlxTypesTimePrimitiveDateTime::check_supported_postgresql_column_type();
    SqlxTypesTimeOffsetDateTime::check_supported_postgresql_column_type();
    SqlxTypesTimeDate::check_supported_postgresql_column_type();
    SqlxTypesTimeTime::check_supported_postgresql_column_type();
    SqlxTypesUuidUuid::check_supported_postgresql_column_type();
    SqlxTypesIpnetworkIpNetwork::check_supported_postgresql_column_type();
    StdNetIpAddr::check_supported_postgresql_column_type();
    SqlxTypesMacAddressMacAddress::check_supported_postgresql_column_type();
    SqlxTypesBitVec::check_supported_postgresql_column_type();
    SqlxTypesJson::<bool>::check_supported_postgresql_column_type();
    SerdeJsonValue::check_supported_postgresql_column_type();
    //
}

pub enum PostgresqlType {
    Bool,
    Char,
    SmallInt,
    SmallSerial,
    Int2,
    Int,
    Serial,
    Int4,
    BigInt,
    BigSerial,
    Int8,
    Real,
    Float4,
    DoublePrecision,
    Float8,
    Varchar,
    CharN,
    Text,
    Name,
    CiText,
    Bytea,
    Interval,
    Int8Range,
    Int4Range,
    TsRange,
    TsTzRange,
    DateRange,
    NumRange,
    Money,
    LTree,
    LQuery,
    Numeric,
    TimestampTz,
    Date,
    Time,
    TimeTz,
    Timestamp,
    Uuid,
    Inet,
    Cidr,
    MacAddr,
    Bit,
    VarBit,
    Json,
    JsonB,
    //maybe Composite types
    //maybe Enumerations
}

impl PostgresqlType {
    pub fn postgresql_naming(&self) -> &str {
        match self {
            Self::Bool => "BOOL",
            Self::Char => "CHAR",
            Self::SmallInt => "SMALLINT",
            Self::SmallSerial => "SMALLSERIAL",
            Self::Int2 => "INT2",
            Self::Int => "INT",
            Self::Serial => "SERIAL",
            Self::Int4 => "INT4",
            Self::BigInt => "BIGINT",
            Self::BigSerial => "BIGSERIAL",
            Self::Int8 => "INT8",
            Self::Real => "REAL",
            Self::Float4 => "FLOAT4",
            Self::DoublePrecision => "DOUBLE PRECISION",
            Self::Float8 => "FLOAT8",
            Self::Varchar => "VARCHAR",
            Self::CharN => "CHAR(N)",
            Self::Text => "TEXT",
            Self::Name => "NAME",
            Self::CiText => "CITEXT",
            Self::Bytea => "BYTEA",
            Self::Interval => "INTERVAL",
            Self::Int8Range => "INT8RANGE",
            Self::Int4Range => "INT4RANGE",
            Self::TsRange => "TSRANGE",
            Self::TsTzRange => "TSTZRANGE",
            Self::DateRange => "DATERANGE",
            Self::NumRange => "NUMRANGE",
            Self::Money => "MONEY",
            Self::LTree => "LTREE",
            Self::LQuery => "LQUERY",
            Self::Numeric => "NUMERIC",
            Self::TimestampTz => "TIMESTAMPTZ",
            Self::Date => "DATE",
            Self::Time => "TIME",
            Self::TimeTz => "TIMETZ",
            Self::Timestamp => "TIMESTAMP",
            Self::Uuid => "UUID",
            Self::Inet => "INET",
            Self::Cidr => "CIDR",
            Self::MacAddr => "MACADDR",
            Self::Bit => "BIT",
            Self::VarBit => "VARBIT",
            Self::Json => "JSON",
            Self::JsonB => "JSONB",
            //maybe Composite types
            //maybe Enumerations
        }
    }
}

pub trait AsPostgresqlBool {}
pub trait AsPostgresqlChar {}
pub trait AsPostgresqlSmallInt {}
pub trait AsPostgresqlSmallSerial {}
pub trait AsPostgresqlInt2 {}
pub trait AsPostgresqlIntSerial {}
pub trait AsPostgresqlInt4 {}
pub trait AsPostgresqlBigInt {}
pub trait AsPostgresqlBigSerial {}
pub trait AsPostgresqlInt8 {}
pub trait AsPostgresqlReal {}
pub trait AsPostgresqlFloat4 {}
pub trait AsPostgresqlDoublePrecision {}
pub trait AsPostgresqlFloat8 {}
pub trait AsPostgresqlVarchar {}
pub trait AsPostgresqlCharN {}
pub trait AsPostgresqlText {}
pub trait AsPostgresqlName {}
pub trait AsPostgresqlCiText {}
pub trait AsPostgresqlBytea {}
pub trait AsPostgresqlInterval {}
pub trait AsPostgresqlInt8Range {}
pub trait AsPostgresqlInt4Range {}
pub trait AsPostgresqlTsRange {}
pub trait AsPostgresqlTsTzRange {}
pub trait AsPostgresqlDateRange {}
pub trait AsPostgresqlNumRange {}
pub trait AsPostgresqlMoney {}
pub trait AsPostgresqlLTree {}
pub trait AsPostgresqlLQuery {}
pub trait AsPostgresqlNumeric {}
pub trait AsPostgresqlTimestampTz {}
pub trait AsPostgresqlDate {}
pub trait AsPostgresqlTime {}
pub trait AsPostgresqlTimeTz {}
pub trait AsPostgresqlTimestamp {}
pub trait AsPostgresqlUuid {}
pub trait AsPostgresqlInet {}
pub trait AsPostgresqlCidr {}
pub trait AsPostgresqlMacAddr {}
pub trait AsPostgresqlBit {}
pub trait AsPostgresqlVarBit {}
pub trait AsPostgresqlJson {}
pub trait AsPostgresqlJsonB {}
