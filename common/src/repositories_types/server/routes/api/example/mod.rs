#[derive(
    Debug,
    postgresql_crud::GeneratePostgresqlCrudSecond
)]
#[postgresql_crud::create_many_additional_error_variants{enum CreateManyAdditionalErrorVariants{}}]
#[postgresql_crud::create_one_additional_error_variants{enum CreateOneAdditionalErrorVariants{}}]
#[postgresql_crud::read_many_additional_error_variants{enum ReadManyAdditionalErrorVariants{}}]
#[postgresql_crud::read_one_additional_error_variants{enum ReadOneAdditionalErrorVariants{}}]
#[postgresql_crud::update_many_additional_error_variants{enum UpdateManyAdditionalErrorVariants{}}]
#[postgresql_crud::update_one_additional_error_variants{enum UpdateOneAdditionalErrorVariants{}}]
#[postgresql_crud::delete_many_additional_error_variants{enum DeleteManyAdditionalErrorVariants{}}]
#[postgresql_crud::delete_one_additional_error_variants{enum DeleteOneAdditionalErrorVariants{}}]
#[postgresql_crud::common_additional_error_variants{
    enum CommonAdditionalErrorVariants {
        CheckCommit {
            #[eo_error_occurence]
            check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
}]
#[postgresql_crud::create_many_additional_route_logic{
    println!("kekw");
}]
#[postgresql_crud::create_one_additional_route_logic{}]
#[postgresql_crud::read_many_additional_route_logic{}]
#[postgresql_crud::read_one_additional_route_logic{}]
#[postgresql_crud::update_many_additional_route_logic{}]
#[postgresql_crud::update_one_additional_route_logic{}]
#[postgresql_crud::delete_many_additional_route_logic{}]
#[postgresql_crud::delete_one_additional_route_logic{}]
#[postgresql_crud::common_additional_route_logic{
    // if let Err(error) = postgresql_crud::check_commit::check_commit(
    //     *app_state.get_enable_api_git_commit_check(),
    //     &headers,
    // ) {
    //     let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
    //     //todo use reserved work instead of TryCreateManyRouteLogicErrorNamed
    //     let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
    //         check_commit: error,
    //         code_occurence: error_occurence_lib::code_occurence!(),
    //     };
    //     eprintln!("{error}");
    //     let mut response = axum::response::IntoResponse::into_response(axum::Json(
    //         TryCreateManyRouteLogicResponseVariants::from(error),
    //     ));
    //     *response.status_mut() = status_code;
    //     return response;
    // }
}]
pub struct Example {
    // pub std_primitive_bool_as_postgresql_bool: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveBoolAsPostgresqlBool,
    pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    // pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI16AsPostgresqlSmallInt,
    // pub std_primitive_i16_as_postgresql_small_int_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    // pub std_primitive_i16_as_postgresql_small_serial: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerial,
    // pub std_primitive_i16_as_postgresql_small_serial_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    // pub std_primitive_i16_as_postgresql_small_int2: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI16AsPostgresqlInt2,
    // pub std_primitive_i16_as_postgresql_small_int2_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull,

    // pub std_primitive_i32_as_postgresql_int: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI32AsPostgresqlInt,
    // pub std_primitive_i32_as_postgresql_int_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI32AsPostgresqlIntNotNull,
    // pub std_primitive_i32_as_postgresql_serial: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI32AsPostgresqlSerial,
    // pub std_primitive_i32_as_postgresql_serial_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI32AsPostgresqlSerialNotNull,
    // pub std_primitive_i32_as_postgresql_int4: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI32AsPostgresqlInt4,
    // pub std_primitive_i32_as_postgresql_int4_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI32AsPostgresqlInt4NotNull,

    // pub std_primitive_i64_as_postgresql_big_int: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI64AsPostgresqlBigInt,
    // pub std_primitive_i64_as_postgresql_big_int_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI64AsPostgresqlBigIntNotNull,
    // pub std_primitive_i64_as_postgresql_big_serial: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerial,
    #[generate_postgresql_crud_second_primary_key]
    pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    // pub std_primitive_i64_as_postgresql_big_int8: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI64AsPostgresqlInt8,
    // pub std_primitive_i64_as_postgresql_big_int8_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveI64AsPostgresqlInt8NotNull,

    // pub std_primitive_f32_as_postgresql_real: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveF32AsPostgresqlReal,
    // pub std_primitive_f32_as_postgresql_real_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveF32AsPostgresqlRealNotNull,
    // pub std_primitive_f32_as_postgresql_float4: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4,
    // pub std_primitive_f32_as_postgresql_float4_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4NotNull,

    // pub std_primitive_f64_as_postgresql_double_precision: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveF64AsPostgresqlDoublePrecision,
    // pub std_primitive_f64_as_postgresql_double_precision_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull,
    // pub std_primitive_f64_as_postgresql_float8: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8,
    // pub std_primitive_f64_as_postgresql_float8_not_null: postgresql_crud::postgresql_types::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8NotNull,

    // pub std_string_string_as_postgresql_varchar: postgresql_crud::postgresql_types::postgresql_type::StdStringStringAsPostgresqlVarchar,
    // pub std_string_string_as_postgresql_varchar_not_null: postgresql_crud::postgresql_types::postgresql_type::StdStringStringAsPostgresqlVarcharNotNull,
    // // pub std_string_string_as_postgresql_char_n: postgresql_crud::postgresql_types::postgresql_type::StdStringStringAsPostgresqlCharN,
    // // pub std_string_string_as_postgresql_char_n_not_null: postgresql_crud::postgresql_types::postgresql_type::StdStringStringAsPostgresqlCharNNotNull,
    // pub std_string_string_as_postgresql_text: postgresql_crud::postgresql_types::postgresql_type::StdStringStringAsPostgresqlText,
    // pub std_string_string_as_postgresql_text_not_null: postgresql_crud::postgresql_types::postgresql_type::StdStringStringAsPostgresqlTextNotNull,
    // // pub std_string_string_as_postgresql_ci_text: postgresql_crud::postgresql_types::postgresql_type::StdStringStringAsPostgresqlCiText,
    // // pub std_string_string_as_postgresql_ci_text_not_null: postgresql_crud::postgresql_types::postgresql_type::StdStringStringAsPostgresqlCiTextNotNull,

    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea: postgresql_crud::postgresql_types::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlBytea,
    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea_not_null: postgresql_crud::postgresql_types::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_money_as_postgresql_money: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
    // pub sqlx_postgres_types_pg_money_as_postgresql_money_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiText,
    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

    // pub sqlx_types_big_decimal_as_postgresql_numeric: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumeric,
    // pub sqlx_types_big_decimal_as_postgresql_numeric_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_decimal_as_postgresql_numeric: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesDecimalAsPostgresqlNumeric,
    // pub sqlx_types_decimal_as_postgresql_numeric_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_chrono_naive_date_as_postgresql_date: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDate,
    // pub sqlx_types_chrono_naive_date_as_postgresql_date_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

    // pub sqlx_types_chrono_naive_time_as_postgresql_time: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTime,
    // pub sqlx_types_chrono_naive_time_as_postgresql_time_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,
    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull,

    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,
    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_time_date_as_postgresql_date: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesTimeDateAsPostgresqlDate,
    // pub sqlx_types_time_date_as_postgresql_date_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNotNull,

    // pub sqlx_types_time_time_as_postgresql_time: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTime,
    // pub sqlx_types_time_time_as_postgresql_time_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    // pub sqlx_types_uuid_uuid_as_postgresql_uuid: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuid,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidNotNull,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough //fails too but primary key is a different logic. need refactor it as different task

    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    // pub std_net_ip_addr_as_postgresql_inet: postgresql_crud::postgresql_types::postgresql_type::StdNetIpAddrAsPostgresqlInet,
    // pub std_net_ip_addr_as_postgresql_inet_not_null: postgresql_crud::postgresql_types::postgresql_type::StdNetIpAddrAsPostgresqlInetNotNull,
    // pub std_net_ip_addr_as_postgresql_cidr: postgresql_crud::postgresql_types::postgresql_type::StdNetIpAddrAsPostgresqlCidr,
    // pub std_net_ip_addr_as_postgresql_cidr_not_null: postgresql_crud::postgresql_types::postgresql_type::StdNetIpAddrAsPostgresqlCidrNotNull,

    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

    // pub sqlx_types_bit_vec_as_postgresql_bit: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesBitVecAsPostgresqlBit,
    // pub sqlx_types_bit_vec_as_postgresql_bit_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNotNull,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesBitVecAsPostgresqlVarBit,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit_not_null: postgresql_crud::postgresql_types::postgresql_type::SqlxTypesBitVecAsPostgresqlVarBitNotNull,

    // pub animal_sqlx_types_json_t_as_postgresql_json: AnimalSqlxTypesJsonAsPostgresqlJson,
    // pub animal_sqlx_types_json_t_as_postgresql_json_not_null: AnimalSqlxTypesJsonAsPostgresqlJsonNotNull,
    // pub animal_sqlx_types_json_t_as_postgresql_json_b: AnimalSqlxTypesJsonAsPostgresqlJsonB,
    // pub animal_sqlx_types_json_t_as_postgresql_json_b_not_null: AnimalSqlxTypesJsonAsPostgresqlJsonBNotNull,
}

//todo enum tree support
//todo generate wrapper type for all possible json type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    // postgresql_crud::GeneratePostgresqlQueryPart
)] //user type must implement utoipa::ToSchema trait
pub struct Animal {
    // pub id: postgresql_crud::json_types::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    pub std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8,
    // pub std_primitive_i16: postgresql_crud::json_types::StdPrimitiveI16,
    // pub std_primitive_i32: postgresql_crud::json_types::StdPrimitiveI32,
    // pub std_primitive_i64: postgresql_crud::json_types::StdPrimitiveI64,
    // pub std_primitive_u8: postgresql_crud::json_types::StdPrimitiveU8,
    // pub std_primitive_u16: postgresql_crud::json_types::StdPrimitiveU16,
    // pub std_primitive_u32: postgresql_crud::json_types::StdPrimitiveU32,
    // pub std_primitive_u64: postgresql_crud::json_types::StdPrimitiveU64,
    // pub std_primitive_f32: postgresql_crud::json_types::StdPrimitiveF32,
    // pub std_primitive_f64: postgresql_crud::json_types::StdPrimitiveF64,
    // pub std_primitive_bool: postgresql_crud::json_types::StdPrimitiveBool,
    // pub std_string_string: postgresql_crud::json_types::StdStringString,
    pub std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8,
    // pub std_option_option_std_primitive_i16: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_primitive_i32: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_primitive_i64: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_primitive_u8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_primitive_u16: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_primitive_u32: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_primitive_u64: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_primitive_f32: postgresql_crud::json_types::StdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_primitive_f64: postgresql_crud::json_types::StdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_primitive_bool: postgresql_crud::json_types::StdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_string_string: postgresql_crud::json_types::StdOptionOptionStdStringString,
    pub std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdPrimitiveI8,
    // pub std_vec_vec_std_primitive_i16: postgresql_crud::json_types::StdVecVecStdPrimitiveI16,
    // pub std_vec_vec_std_primitive_i32: postgresql_crud::json_types::StdVecVecStdPrimitiveI32,
    // pub std_vec_vec_std_primitive_i64: postgresql_crud::json_types::StdVecVecStdPrimitiveI64,
    // pub std_vec_vec_std_primitive_u8: postgresql_crud::json_types::StdVecVecStdPrimitiveU8,
    // pub std_vec_vec_std_primitive_u16: postgresql_crud::json_types::StdVecVecStdPrimitiveU16,
    // pub std_vec_vec_std_primitive_u32: postgresql_crud::json_types::StdVecVecStdPrimitiveU32,
    // pub std_vec_vec_std_primitive_u64: postgresql_crud::json_types::StdVecVecStdPrimitiveU64,
    // pub std_vec_vec_std_primitive_f32: postgresql_crud::json_types::StdVecVecStdPrimitiveF32,
    // pub std_vec_vec_std_primitive_f64: postgresql_crud::json_types::StdVecVecStdPrimitiveF64,
    // pub std_vec_vec_std_primitive_bool: postgresql_crud::json_types::StdVecVecStdPrimitiveBool,
    // pub std_vec_vec_std_string_string: postgresql_crud::json_types::StdVecVecStdStringString,
    pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8,
    // pub std_option_option_std_vec_vec_std_primitive_i16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_primitive_i32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_primitive_i64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_primitive_u8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_primitive_u16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_primitive_u32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_primitive_u64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_primitive_f32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_primitive_f64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_primitive_bool: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_string_string: postgresql_crud::json_types::StdOptionOptionStdVecVecStdStringString,
    pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8,
    // pub std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_vec_vec_std_option_option_std_string_string: postgresql_crud::json_types::StdVecVecStdOptionOptionStdStringString,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_option_option_std_string_string: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdStringString,

    // pub object: ObjectDoggie,
    // pub std_option_option_object: StdOptionOptionObjectDoggie,

    // pub std_vec_vec_object_with_id: StdVecVecObjectWithIdDoggie,
    // pub std_option_option_std_vec_vec_object_with_id: StdOptionOptionStdVecVecObjectWithIdDoggie
}
/////////////////////////////////////////
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
struct AnimalToCreateOrigin {
    std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8ToCreate,
    std_option_option_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8ToCreate,
    std_vec_vec_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8ToCreate,
    std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8ToCreate,
    std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
    std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalToCreateOrigin {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl AnimalToCreateOrigin {
    fn try_generate_postgresql_query_part_to_create(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_vec_vec_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_vec_vec_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_option_option_std_vec_vec_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_vec_vec_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_vec_vec_std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_vec_vec_std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.std_option_option_std_vec_vec_std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_vec_vec_std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_option_option_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_vec_vec_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_option_option_std_vec_vec_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_vec_vec_std_option_option_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_option_option_std_vec_vec_std_option_option_std_primitive_i8, query);
        query
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct AnimalToCreateWithGeneratedId(AnimalToCreateOrigin);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct AnimalToCreateWithoutGeneratedId(AnimalToCreateOrigin);
impl AnimalToCreateWithGeneratedId {
    pub fn new(
        std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8ToCreate,
        std_option_option_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8ToCreate,
        std_vec_vec_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8ToCreate,
        std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8ToCreate,
        std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
        std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
    ) -> Self {
        Self(AnimalToCreateOrigin {
            std_primitive_i8,
            std_option_option_std_primitive_i8,
            std_vec_vec_std_primitive_i8,
            std_option_option_std_vec_vec_std_primitive_i8,
            std_vec_vec_std_option_option_std_primitive_i8,
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        })
    }
}
impl AnimalToCreateWithoutGeneratedId {
    pub fn new(
        std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8ToCreate,
        std_option_option_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8ToCreate,
        std_vec_vec_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8ToCreate,
        std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8ToCreate,
        std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
        std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8ToCreate,
    ) -> Self {
        Self(AnimalToCreateOrigin {
            std_primitive_i8,
            std_option_option_std_primitive_i8,
            std_vec_vec_std_primitive_i8,
            std_option_option_std_vec_vec_std_primitive_i8,
            std_vec_vec_std_option_option_std_primitive_i8,
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        })
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalToCreateWithGeneratedId {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalToCreateWithoutGeneratedId {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl AnimalToCreateWithGeneratedId {
    fn try_generate_postgresql_query_part_to_create(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        match self.0.try_generate_postgresql_query_part_to_create(increment) {
            Ok(value) => Ok(format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{value}")),
            Err(error) => Err(error),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.0.bind_value_to_postgresql_query_part_to_create(query);
        query
    }
}
impl AnimalToCreateWithoutGeneratedId {
    fn try_generate_postgresql_query_part_to_create(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        match self.0.try_generate_postgresql_query_part_to_create(increment) {
            Ok(value) => Ok(value),
            Err(error) => Err(error),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.0.bind_value_to_postgresql_query_part_to_create(query);
        query
    }
}
impl<'a> postgresql_crud::BindQuery<'a> for AnimalToCreateWithoutGeneratedId {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_vec_vec_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_vec_vec_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_option_option_std_vec_vec_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_vec_vec_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_vec_vec_std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_vec_vec_std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_option_option_std_vec_vec_std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_vec_vec_std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_option_option_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_vec_vec_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_option_option_std_vec_vec_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_vec_vec_std_option_option_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_option_option_std_vec_vec_std_option_option_std_primitive_i8, query);
        query
    }
}
impl<'a> postgresql_crud::BindQuerySecond<'a> for AnimalToCreateWithoutGeneratedId {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_vec_vec_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_vec_vec_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_option_option_std_vec_vec_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_vec_vec_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_vec_vec_std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_vec_vec_std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_create(&self.0.std_option_option_std_vec_vec_std_option_option_std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_option_option_std_vec_vec_std_option_option_std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_option_option_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_vec_vec_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_option_option_std_vec_vec_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_vec_vec_std_option_option_std_primitive_i8, query);
        query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_option_option_std_vec_vec_std_option_option_std_primitive_i8, query);
        query
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToCreateTraits<'_> for AnimalToCreateWithoutGeneratedId {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToReadWithoutId {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
    StdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
    StdVecVecStdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
    StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToReadWithId {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id(postgresql_crud::json_types::PostgresqlJsonTypeUuidFieldReader),
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
    StdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
    StdVecVecStdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
    StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToReadWithoutId {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalFieldToReadWithoutId {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalFieldToReadWithoutId::StdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithoutId::StdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithoutId::StdVecVecStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithoutId::StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToReadWithId {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalFieldToReadWithId {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalFieldToReadWithId::Id(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdVecVecStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeAnimalOptionsToReadWithoutId {
    #[serde(skip_serializing_if = "Option::is_none")]
    std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeAnimalOptionsToReadWithId {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeUuidOptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeAnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl PostgresqlJsonTypeAnimalOptionsToReadWithoutId {
    pub fn try_new(
        std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>,
        std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionsToRead>>,
        std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionsToRead>>,
        std_option_option_std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>,
        std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
        std_option_option_std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    ) -> Result<Self, PostgresqlJsonTypeAnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed> {
        if let (None, None, None, None, None, None) = (
            &std_primitive_i8,
            &std_option_option_std_primitive_i8,
            &std_vec_vec_std_primitive_i8,
            &std_option_option_std_vec_vec_std_primitive_i8,
            &std_vec_vec_std_option_option_std_primitive_i8,
            &std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        ) {
            return Err(PostgresqlJsonTypeAnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self {
            std_primitive_i8,
            std_option_option_std_primitive_i8,
            std_vec_vec_std_primitive_i8,
            std_option_option_std_vec_vec_std_primitive_i8,
            std_vec_vec_std_option_option_std_primitive_i8,
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        })
    }
}
impl PostgresqlJsonTypeAnimalOptionsToReadWithId {
    pub fn try_new(
        id: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeUuidOptionsToRead>>,
        std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>,
        std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionsToRead>>,
        std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionsToRead>>,
        std_option_option_std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>,
        std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
        std_option_option_std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    ) -> Result<Self, PostgresqlJsonTypeAnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed> {
        if let (None, None, None, None, None, None, None) = (
            &id,
            &std_primitive_i8,
            &std_option_option_std_primitive_i8,
            &std_vec_vec_std_primitive_i8,
            &std_option_option_std_vec_vec_std_primitive_i8,
            &std_vec_vec_std_option_option_std_primitive_i8,
            &std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        ) {
            return Err(PostgresqlJsonTypeAnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self {
            id,
            std_primitive_i8,
            std_option_option_std_primitive_i8,
            std_vec_vec_std_primitive_i8,
            std_option_option_std_vec_vec_std_primitive_i8,
            std_vec_vec_std_option_option_std_primitive_i8,
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8,
        })
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeAnimalOptionsToReadWithoutId {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __field3,
            __field4,
            __field5,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    3u64 => serde::__private::Ok(__Field::__field3),
                    4u64 => serde::__private::Ok(__Field::__field4),
                    5u64 => serde::__private::Ok(__Field::__field5),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "std_primitive_i8" => serde::__private::Ok(__Field::__field0),
                    "std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field1),
                    "std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field2),
                    "std_option_option_std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field3),
                    "std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field4),
                    "std_option_option_std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field5),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"std_primitive_i8" => serde::__private::Ok(__Field::__field0),
                    b"std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field1),
                    b"std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field2),
                    b"std_option_option_std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field3),
                    b"std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field4),
                    b"std_option_option_std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field5),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PostgresqlJsonTypeAnimalOptionsToReadWithoutId>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeAnimalOptionsToReadWithoutId;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeAnimalOptionsToRead")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 6 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 6 elements"));
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 6 elements"));
                    }
                };
                let __field3 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 6 elements"));
                    }
                };
                let __field4 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 6 elements"));
                    }
                };
                let __field5 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 6 elements"));
                    }
                };
                match PostgresqlJsonTypeAnimalOptionsToReadWithoutId::try_new(__field0, __field1, __field2, __field3, __field4, __field5) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field3: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field4: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field5: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i8"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_primitive_i8"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionsToRead>>>(
                                &mut __map,
                            )?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_std_primitive_i8"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field3 => {
                            if serde::__private::Option::is_some(&__field3) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_std_primitive_i8"));
                            }
                            __field3 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>>(
                                &mut __map,
                            )?);
                        }
                        __Field::__field4 => {
                            if serde::__private::Option::is_some(&__field4) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_std_option_option_std_primitive_i8"));
                            }
                            __field4 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(
                                &mut __map,
                            )?);
                        }
                        __Field::__field5 => {
                            if serde::__private::Option::is_some(&__field5) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"));
                            }
                            __field5 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
                            >(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("std_primitive_i8")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_primitive_i8")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("std_vec_vec_std_primitive_i8")?,
                };
                let __field3 = match __field3 {
                    serde::__private::Some(__field3) => __field3,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_std_primitive_i8")?,
                };
                let __field4 = match __field4 {
                    serde::__private::Some(__field4) => __field4,
                    serde::__private::None => serde::__private::de::missing_field("std_vec_vec_std_option_option_std_primitive_i8")?,
                };
                let __field5 = match __field5 {
                    serde::__private::Some(__field5) => __field5,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8")?,
                };
                match PostgresqlJsonTypeAnimalOptionsToReadWithoutId::try_new(__field0, __field1, __field2, __field3, __field4, __field5) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &[
            "std_primitive_i8",
            "std_option_option_std_primitive_i8",
            "std_vec_vec_std_primitive_i8",
            "std_option_option_std_vec_vec_std_primitive_i8",
            "std_vec_vec_std_option_option_std_primitive_i8",
            "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
        ];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "PostgresqlJsonTypeAnimalOptionsToReadWithoutId",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeAnimalOptionsToReadWithoutId>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeAnimalOptionsToReadWithId {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __field3,
            __field4,
            __field5,
            __field6,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    3u64 => serde::__private::Ok(__Field::__field3),
                    4u64 => serde::__private::Ok(__Field::__field4),
                    5u64 => serde::__private::Ok(__Field::__field5),
                    6u64 => serde::__private::Ok(__Field::__field6),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "id" => serde::__private::Ok(__Field::__field0),
                    "std_primitive_i8" => serde::__private::Ok(__Field::__field1),
                    "std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field2),
                    "std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field3),
                    "std_option_option_std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field4),
                    "std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field5),
                    "std_option_option_std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field6),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"id" => serde::__private::Ok(__Field::__field0),
                    b"std_primitive_i8" => serde::__private::Ok(__Field::__field1),
                    b"std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field2),
                    b"std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field3),
                    b"std_option_option_std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field4),
                    b"std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field5),
                    b"std_option_option_std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field6),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PostgresqlJsonTypeAnimalOptionsToReadWithId>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeAnimalOptionsToReadWithId;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeAnimalOptionsToRead")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeUuidOptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field3 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field4 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field5 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 7 elements"));
                    }
                };
                let __field6 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 7 elements"));
                    }
                };
                match PostgresqlJsonTypeAnimalOptionsToReadWithId::try_new(__field0, __field1, __field2, __field3, __field4, __field5, __field6) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeUuidOptionsToRead>>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field3: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field4: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field5: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                let mut __field6: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("id"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeUuidOptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i8"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_primitive_i8"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionsToRead>>>(
                                &mut __map,
                            )?);
                        }
                        __Field::__field3 => {
                            if serde::__private::Option::is_some(&__field3) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_std_primitive_i8"));
                            }
                            __field3 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field4 => {
                            if serde::__private::Option::is_some(&__field4) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_std_primitive_i8"));
                            }
                            __field4 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>>(
                                &mut __map,
                            )?);
                        }
                        __Field::__field5 => {
                            if serde::__private::Option::is_some(&__field5) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_std_option_option_std_primitive_i8"));
                            }
                            __field5 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>>(
                                &mut __map,
                            )?);
                        }
                        __Field::__field6 => {
                            if serde::__private::Option::is_some(&__field6) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"));
                            }
                            __field6 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
                            >(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("id")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("std_primitive_i8")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_primitive_i8")?,
                };
                let __field3 = match __field3 {
                    serde::__private::Some(__field3) => __field3,
                    serde::__private::None => serde::__private::de::missing_field("std_vec_vec_std_primitive_i8")?,
                };
                let __field4 = match __field4 {
                    serde::__private::Some(__field4) => __field4,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_std_primitive_i8")?,
                };
                let __field5 = match __field5 {
                    serde::__private::Some(__field5) => __field5,
                    serde::__private::None => serde::__private::de::missing_field("std_vec_vec_std_option_option_std_primitive_i8")?,
                };
                let __field6 = match __field6 {
                    serde::__private::Some(__field6) => __field6,
                    serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8")?,
                };
                match PostgresqlJsonTypeAnimalOptionsToReadWithId::try_new(__field0, __field1, __field2, __field3, __field4, __field5, __field6) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &[
            "id",
            "std_primitive_i8",
            "std_option_option_std_primitive_i8",
            "std_vec_vec_std_primitive_i8",
            "std_option_option_std_vec_vec_std_primitive_i8",
            "std_vec_vec_std_option_option_std_primitive_i8",
            "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
        ];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "PostgresqlJsonTypeAnimalOptionsToReadWithId",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeAnimalOptionsToReadWithId>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeAnimalOptionsToReadWithoutId {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_vec_vec_std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        }
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeAnimalOptionsToReadWithId {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            id: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_vec_vec_std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToUpdate {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8,
    #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
    StdOptionOptionStdPrimitiveI8,
    #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
    StdVecVecStdPrimitiveI8,
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdPrimitiveI8,
    #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
    StdVecVecStdOptionOptionStdPrimitiveI8,
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        match &self {
            Self::StdPrimitiveI8 => "std_primitive_i8".to_owned(),
            Self::StdOptionOptionStdPrimitiveI8 => "std_option_option_std_primitive_i8".to_owned(),
            Self::StdVecVecStdPrimitiveI8 => "std_vec_vec_std_primitive_i8".to_owned(),
            Self::StdOptionOptionStdVecVecStdPrimitiveI8 => "std_option_option_std_vec_vec_std_primitive_i8".to_owned(),
            Self::StdVecVecStdOptionOptionStdPrimitiveI8 => "std_vec_vec_std_option_option_std_primitive_i8".to_owned(),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => "std_option_option_std_vec_vec_std_option_option_std_primitive_i8".to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum PostgresqlJsonTypeAnimalOptionToUpdateOrigin {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionToUpdate>),
    #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
    StdOptionOptionStdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionToUpdate>),
    #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
    StdVecVecStdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionToUpdate>),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionToUpdate>),
    #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
    StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdate>),
    #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value<postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdate>),
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Create {
        #[eo_error_occurence]
        error: postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeAnimalOptionToUpdateOrigin {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalOptionsToUpdate {
    pub id: postgresql_crud::json_types::PostgresqlJsonTypeUuidOptionToUpdate,
    pub fields: PostgresqlJsonTypeAnimalOptionToUpdate,
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalOptionsToUpdate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            fields: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl std::fmt::Display for Animal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", &self)
    }
}
pub type PostgresqlJsonTypeAnimalToCreate = AnimalToCreateWithoutGeneratedId;
pub type AnimalFieldToRead = AnimalFieldToReadWithoutId;
pub type PostgresqlJsonTypeAnimalOptionsToRead = PostgresqlJsonTypeAnimalOptionsToReadWithoutId;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeAnimalFieldReader(std::vec::Vec<AnimalFieldToReadWithoutId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeAnimalFieldReaderTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithoutId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeAnimalFieldReader {
    pub fn try_new(value: std::vec::Vec<AnimalFieldToReadWithoutId>) -> Result<Self, PostgresqlJsonTypeAnimalFieldReaderTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlJsonTypeAnimalFieldReaderTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(PostgresqlJsonTypeAnimalFieldReaderTryNewErrorNamed::NotUniqueFieldFilter {
                    field: element,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            } else {
                unique.push(element);
            }
        }
        Ok(Self(unique))
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeAnimalFieldReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PostgresqlJsonTypeAnimalFieldReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeAnimalFieldReader;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct PostgresqlJsonTypeAnimalFieldReader")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalFieldToReadWithoutId> = <std::vec::Vec<AnimalFieldToReadWithoutId> as serde::Deserialize>::deserialize(__e)?;
                match PostgresqlJsonTypeAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalFieldToReadWithoutId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlJsonTypeAnimalFieldReader with 1 element"));
                    }
                };
                match PostgresqlJsonTypeAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "PostgresqlJsonTypeAnimalFieldReader",
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeAnimalFieldReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeAnimalFieldReader {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl postgresql_crud::GeneratePostgresqlQueryPartToRead for AnimalFieldToReadWithoutId {
    fn generate_postgresql_query_part_to_read_from_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
        let mut acc = std::string::String::default();
        for element in value {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithoutId::StdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_primitive_i8", column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_option_option_std_primitive_i8", column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message),
                    AnimalFieldToReadWithoutId::StdVecVecStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_vec_vec_std_primitive_i8", column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_primitive_i8",
                        column_name_and_maybe_field_getter,
                        column_name_and_maybe_field_getter_for_error_message
                    ),
                    AnimalFieldToReadWithoutId::StdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_option_option_std_primitive_i8",
                        column_name_and_maybe_field_getter,
                        column_name_and_maybe_field_getter_for_error_message
                    ),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                        column_name_and_maybe_field_getter,
                        column_name_and_maybe_field_getter_for_error_message
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        format!("{acc}")
    }
}
pub type AnimalReader = PostgresqlJsonTypeAnimalOptionsToRead;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeAnimalOptionToUpdate(std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed {
    FieldsAreEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdOptionOptionStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdVecVecStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdOptionOptionStdVecVecStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeAnimalOptionToUpdate {
    pub fn try_new(value: std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin>) -> Result<Self, PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed::FieldsAreEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            let generate_not_unique_field = |value: &std::primitive::str| format!("not unique {value} field");
            for element in &value {
                match element {
                    PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdPrimitiveI8 {
                                error: generate_not_unique_field("std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                    PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdOptionOptionStdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdOptionOptionStdPrimitiveI8 {
                                error: generate_not_unique_field("std_option_option_std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                    PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdVecVecStdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdVecVecStdPrimitiveI8 {
                                error: generate_not_unique_field("std_vec_vec_std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                    PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdOptionOptionStdVecVecStdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdOptionOptionStdVecVecStdPrimitiveI8 {
                                error: generate_not_unique_field("std_option_option_std_vec_vec_std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                    PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdVecVecStdOptionOptionStdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdVecVecStdOptionOptionStdPrimitiveI8 {
                                error: generate_not_unique_field("std_vec_vec_std_option_option_std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                    PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
                                error: generate_not_unique_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                }
            }
        }
        Ok(Self(value))
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeAnimalOptionToUpdate {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PostgresqlJsonTypeAnimalOptionToUpdate>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeAnimalOptionToUpdate;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct PostgresqlJsonTypeAnimalOptionToUpdate")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin> = <std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin> as serde::Deserialize>::deserialize(__e)?;
                match PostgresqlJsonTypeAnimalOptionToUpdate::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlJsonTypeAnimalOptionToUpdate with 1 element"));
                    }
                };
                match PostgresqlJsonTypeAnimalOptionToUpdate::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "PostgresqlJsonTypeAnimalOptionToUpdate",
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeAnimalOptionToUpdate>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeAnimalOptionToUpdate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeAnimalOptionToUpdate {
    fn try_generate_postgresql_query_part_to_update(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        let generate_jsonb_set_path = |value: &std::primitive::str| {
            let previous = match jsonb_set_path.is_empty() {
                true => std::string::String::default(),
                false => format!("{jsonb_set_path},"),
            };
            format!("{previous}{value}")
        };
        let mut local_acc = format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end)");
        for element in &self.0 {
            match &element {
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                    match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(&value.value, &local_acc, &generate_jsonb_set_target("std_primitive_i8"), &generate_jsonb_set_path("std_primitive_i8"), increment) {
                        Ok(value) => {
                            local_acc = value;
                        }
                        Err(error) => {
                            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdPrimitiveI8 {
                                error,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                }
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                    &value.value,
                    &local_acc,
                    &generate_jsonb_set_target("std_option_option_std_primitive_i8"),
                    &generate_jsonb_set_path("std_option_option_std_primitive_i8"),
                    increment,
                ) {
                    Ok(value) => {
                        local_acc = value;
                    }
                    Err(error) => {
                        return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdPrimitiveI8 {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                },
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                    &value.value,
                    &local_acc,
                    &generate_jsonb_set_target("std_vec_vec_std_primitive_i8"),
                    &generate_jsonb_set_path("std_vec_vec_std_primitive_i8"),
                    increment,
                ) {
                    Ok(value) => {
                        local_acc = value;
                    }
                    Err(error) => {
                        return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdPrimitiveI8 {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                },
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                    &value.value,
                    &local_acc,
                    &generate_jsonb_set_target("std_option_option_std_vec_vec_std_primitive_i8"),
                    &generate_jsonb_set_path("std_option_option_std_vec_vec_std_primitive_i8"),
                    increment,
                ) {
                    Ok(value) => {
                        local_acc = value;
                    }
                    Err(error) => {
                        return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdPrimitiveI8 {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                },
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                    &value.value,
                    &local_acc,
                    &generate_jsonb_set_target("std_vec_vec_std_option_option_std_primitive_i8"),
                    &generate_jsonb_set_path("std_vec_vec_std_option_option_std_primitive_i8"),
                    increment,
                ) {
                    Ok(value) => {
                        local_acc = value;
                    }
                    Err(error) => {
                        return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdOptionOptionStdPrimitiveI8 {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                },
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                    match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                        &value.value,
                        &local_acc,
                        &generate_jsonb_set_target("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"),
                        &generate_jsonb_set_path("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"),
                        increment,
                    ) {
                        Ok(value) => {
                            local_acc = value;
                        }
                        Err(error) => {
                            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
                                error,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                }
            }
        }
        Ok(local_acc)
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0 {
            match element {
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                    query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
            }
        }
        query
    }
}
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlJsonTypeAnimalOptionToUpdate {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToUpdateTraits<'_> for PostgresqlJsonTypeAnimalOptionToUpdate {}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectWithIdAnimal {
    pub id: postgresql_crud::json_types::PostgresqlJsonTypeUuidOptionToUpdate,
    pub std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8,
    pub std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8,
    pub std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdPrimitiveI8,
    pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8,
    pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ObjectWithIdAnimal {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectAnimal {
    pub std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8,
    pub std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8,
    pub std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdPrimitiveI8,
    pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8,
    pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
}
impl postgresql_crud::PostgresqlJsonType for ObjectAnimal {
    type PostgresqlJsonTypeSelfToCreate<'a> = PostgresqlJsonTypeObjectAnimalToCreate;
    fn try_generate_postgresql_query_part_to_create(postgresql_json_type_self_to_create: &Self::PostgresqlJsonTypeSelfToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        postgresql_json_type_self_to_create.try_generate_postgresql_query_part_to_create(increment)
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(postgresql_json_type_self_to_create: Self::PostgresqlJsonTypeSelfToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        postgresql_json_type_self_to_create.bind_value_to_postgresql_query_part_to_create(query)
    }
    type PostgresqlJsonTypeSelfFieldReader<'a> = PostgresqlJsonTypeObjectAnimalFieldReader;
    type PostgresqlJsonTypeSelfOptionsToRead<'a> = PostgresqlJsonTypeObjectAnimalOptionsToRead;
    fn generate_postgresql_query_part_to_read(
        postgresql_json_type_self_field_reader: &Self::PostgresqlJsonTypeSelfFieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
    ) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in &postgresql_json_type_self_field_reader.0 {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithoutId::StdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_primitive_i8", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))")
    }
    type PostgresqlJsonTypeSelfOptionToUpdate<'a> = PostgresqlJsonTypeObjectAnimalOptionToUpdate;
    type PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = PostgresqlJsonTypeObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed;
    fn try_generate_postgresql_query_part_to_update(
        postgresql_json_type_self_option_to_update: &Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
        postgresql_json_type_self_option_to_update.try_generate_postgresql_query_part_to_update(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment)
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(
        postgresql_json_type_self_option_to_update: Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        postgresql_json_type_self_option_to_update.bind_value_to_postgresql_query_part_to_update(query)
    }
}
pub type PostgresqlJsonTypeObjectAnimalToCreate = AnimalToCreateWithoutGeneratedId;
pub type PostgresqlJsonTypeObjectAnimalOptionsToRead = PostgresqlJsonTypeAnimalOptionsToReadWithoutId;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeObjectAnimalFieldReader(std::vec::Vec<AnimalFieldToReadWithoutId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeObjectAnimalPostgresqlJsonTypeFieldReaderTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithoutId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeObjectAnimalFieldReader {
    pub fn try_new(value: std::vec::Vec<AnimalFieldToReadWithoutId>) -> Result<Self, PostgresqlJsonTypeObjectAnimalPostgresqlJsonTypeFieldReaderTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlJsonTypeObjectAnimalPostgresqlJsonTypeFieldReaderTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(PostgresqlJsonTypeObjectAnimalPostgresqlJsonTypeFieldReaderTryNewErrorNamed::NotUniqueFieldFilter {
                    field: element,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            } else {
                unique.push(element);
            }
        }
        Ok(Self(unique))
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeObjectAnimalFieldReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PostgresqlJsonTypeObjectAnimalFieldReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeObjectAnimalFieldReader;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct PostgresqlJsonTypeObjectAnimalFieldReader")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalFieldToReadWithoutId> = <std::vec::Vec<AnimalFieldToReadWithoutId> as serde::Deserialize>::deserialize(__e)?;
                match PostgresqlJsonTypeObjectAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalFieldToReadWithoutId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlJsonTypeObjectAnimalFieldReader with 1 element"));
                    }
                };
                match PostgresqlJsonTypeObjectAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "PostgresqlJsonTypeObjectAnimalFieldReader",
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeObjectAnimalFieldReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeObjectAnimalFieldReader {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
pub type ObjectAnimalReader = PostgresqlJsonTypeObjectAnimalOptionsToRead;
pub type PostgresqlJsonTypeObjectAnimalOptionToUpdate = PostgresqlJsonTypeAnimalOptionToUpdate;
pub type PostgresqlJsonTypeObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed;
pub type PostgresqlJsonTypeObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamedWithSerializeDeserialize = PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamedWithSerializeDeserialize;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ObjectAnimalPostgresqlJson;
impl std::fmt::Display for ObjectAnimalPostgresqlJson {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for ObjectAnimalPostgresqlJson {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl<'a> postgresql_crud::BindQuerySecond<'a> for ObjectAnimalPostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::CreateTableQueryPart for ObjectAnimalPostgresqlJson {
    fn create_table_query_part() -> impl std::fmt::Display {
        "JSONB"
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ObjectAnimalPostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToCreateTraits<'_> for ObjectAnimalPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeObjectAnimalColumnPostgresqlJson;
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeObjectAnimalColumnPostgresqlJson {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        todo!()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeObjectAnimalColumnPostgresqlJson {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        todo!()
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeObjectAnimalColumnPostgresqlJson {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![]
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeObjectAnimalColumnPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeObjectAnimalToCreatePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeObjectAnimalToCreatePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeObjectAnimalToCreatePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToCreateTraits<'_> for PostgresqlTypeObjectAnimalToCreatePostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeObjectAnimalToReadPostgresqlJson;
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeObjectAnimalToReadPostgresqlJson {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        todo!()
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeObjectAnimalToReadPostgresqlJson {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeObjectAnimalToReadPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeObjectAnimalToUpdatePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeObjectAnimalToUpdatePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeObjectAnimalToUpdatePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToUpdateTraits<'_> for PostgresqlTypeObjectAnimalToUpdatePostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeObjectAnimalWherePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeObjectAnimalWherePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeObjectAnimalWherePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfWhereTraits<'_> for PostgresqlTypeObjectAnimalWherePostgresqlJson {}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlType<'_> for ObjectAnimalPostgresqlJson {
    type PostgresqlTypeSelf = ObjectAnimalPostgresqlJson;
    type PostgresqlTypeSelfColumn = PostgresqlTypeObjectAnimalColumnPostgresqlJson;
    type PostgresqlTypeSelfToCreate = PostgresqlTypeObjectAnimalToCreatePostgresqlJson;
    type PostgresqlTypeSelfToRead = PostgresqlTypeObjectAnimalToReadPostgresqlJson;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeObjectAnimalToUpdatePostgresqlJson;
    type PostgresqlTypeSelfWhere = PostgresqlTypeObjectAnimalWherePostgresqlJson;
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdOptionOptionObjectAnimal(pub std::option::Option<ObjectAnimal>);
impl postgresql_crud::PostgresqlJsonType for StdOptionOptionObjectAnimal {
    type PostgresqlJsonTypeSelfToCreate<'a> = PostgresqlJsonTypeStdOptionOptionObjectAnimalToCreate;
    fn try_generate_postgresql_query_part_to_create(postgresql_json_type_self_to_create: &Self::PostgresqlJsonTypeSelfToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        match &postgresql_json_type_self_to_create.0 {
            Some(value) => match value.try_generate_postgresql_query_part_to_create(increment) {
                Ok(value) => Ok(value),
                Err(error) => Err(error),
            },
            None => Ok(std::string::String::from("null")),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(postgresql_json_type_self_to_create: Self::PostgresqlJsonTypeSelfToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let Some(value) = postgresql_json_type_self_to_create.0 {
            query = value.bind_value_to_postgresql_query_part_to_create(query);
        }
        query
    }
    type PostgresqlJsonTypeSelfFieldReader<'a> = PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader;
    type PostgresqlJsonTypeSelfOptionsToRead<'a> = PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionsToRead;
    fn generate_postgresql_query_part_to_read(
        postgresql_json_type_self_field_reader: &Self::PostgresqlJsonTypeSelfFieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
    ) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in &postgresql_json_type_self_field_reader.0 {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithoutId::StdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_primitive_i8", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithoutId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        format!("jsonb_build_object('{field_ident}', case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('value', null) else jsonb_build_object('value',{acc}) end)")
    }
    type PostgresqlJsonTypeSelfOptionToUpdate<'a> = PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdate;
    type PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed;
    fn try_generate_postgresql_query_part_to_update(
        postgresql_json_type_self_option_to_update: &Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
        Ok(match &postgresql_json_type_self_option_to_update.0 {
            Some(value) => {
                let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
                let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
                for element in &value.0 {
                    match element {
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("std_primitive_i8"), "std_primitive_i8", increment) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                &value.value,
                                &std_option_option_object_acc,
                                &generate_jsonb_set_target("std_option_option_std_primitive_i8"),
                                "std_option_option_std_primitive_i8",
                                increment,
                            ) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                &value.value,
                                &std_option_option_object_acc,
                                &generate_jsonb_set_target("std_vec_vec_std_primitive_i8"),
                                "std_vec_vec_std_primitive_i8",
                                increment,
                            ) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                &value.value,
                                &std_option_option_object_acc,
                                &generate_jsonb_set_target("std_option_option_std_vec_vec_std_primitive_i8"),
                                "std_option_option_std_vec_vec_std_primitive_i8",
                                increment,
                            ) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                &value.value,
                                &std_option_option_object_acc,
                                &generate_jsonb_set_target("std_vec_vec_std_option_option_std_primitive_i8"),
                                "std_vec_vec_std_option_option_std_primitive_i8",
                                increment,
                            ) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdVecVecStdOptionOptionStdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                            match <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_query_part_to_update(
                                &value.value,
                                &std_option_option_object_acc,
                                &generate_jsonb_set_target("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"),
                                "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                                increment,
                            ) {
                                Ok(value) => {
                                    std_option_option_object_acc = value;
                                }
                                Err(error) => {
                                    return Err(PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
                                        error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                    }
                }
                format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',{std_option_option_object_acc})")
            }
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})")
                }
                None => {
                    return Err(PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                }
            },
        })
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(
        postgresql_json_type_self_option_to_update: Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match postgresql_json_type_self_option_to_update.0 {
            Some(value) => {
                for element in value.0 {
                    match element {
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                        PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
                            query = <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                        }
                    }
                }
            }
            None => {
                query = query.bind(sqlx::types::Json(None::<std::option::Option<std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin>>>));
            }
        }
        query
    }
}
pub type StdOptionOptionObjectAnimalToCreateOrigin = AnimalToCreateWithoutGeneratedId;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeStdOptionOptionObjectAnimalToCreate(pub std::option::Option<StdOptionOptionObjectAnimalToCreateOrigin>);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdOptionOptionObjectAnimalToCreate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionsToRead(pub std::option::Option<PostgresqlJsonTypeAnimalOptionsToReadWithoutId>);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionsToRead {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader(std::vec::Vec<AnimalFieldToReadWithoutId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReaderTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithoutId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader {
    pub fn try_new(value: std::vec::Vec<AnimalFieldToReadWithoutId>) -> Result<Self, PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReaderTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReaderTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReaderTryNewErrorNamed::NotUniqueFieldFilter {
                    field: element,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            } else {
                unique.push(element);
            }
        }
        Ok(Self(unique))
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalFieldToReadWithoutId> = <std::vec::Vec<AnimalFieldToReadWithoutId> as serde::Deserialize>::deserialize(__e)?;
                match PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalFieldToReadWithoutId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader with 1 element"));
                    }
                };
                match PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader",
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdOptionOptionObjectAnimalFieldReader {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
pub type StdOptionOptionObjectAnimalReader = PostgresqlJsonTypeAnimalOptionsToReadWithoutId;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdate(pub std::option::Option<PostgresqlJsonTypeAnimalOptionToUpdate>);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeStdOptionOptionObjectAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::json_types::PostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdOptionOptionObjectAnimalPostgresqlJson;
impl std::fmt::Display for StdOptionOptionObjectAnimalPostgresqlJson {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for StdOptionOptionObjectAnimalPostgresqlJson {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl<'a> postgresql_crud::BindQuerySecond<'a> for StdOptionOptionObjectAnimalPostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::CreateTableQueryPart for StdOptionOptionObjectAnimalPostgresqlJson {
    fn create_table_query_part() -> impl std::fmt::Display {
        "JSONB"
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionObjectAnimalPostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToCreateTraits<'_> for StdOptionOptionObjectAnimalPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionObjectAnimalColumnPostgresqlJson;
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdOptionOptionObjectAnimalColumnPostgresqlJson {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        todo!()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdOptionOptionObjectAnimalColumnPostgresqlJson {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        todo!()
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionObjectAnimalColumnPostgresqlJson {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![]
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeStdOptionOptionObjectAnimalColumnPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionObjectAnimalToCreatePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeStdOptionOptionObjectAnimalToCreatePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionObjectAnimalToCreatePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToCreateTraits<'_> for PostgresqlTypeStdOptionOptionObjectAnimalToCreatePostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionObjectAnimalToReadPostgresqlJson;
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdOptionOptionObjectAnimalToReadPostgresqlJson {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        todo!()
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdOptionOptionObjectAnimalToReadPostgresqlJson {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeStdOptionOptionObjectAnimalToReadPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionObjectAnimalToUpdatePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeStdOptionOptionObjectAnimalToUpdatePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionObjectAnimalToUpdatePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToUpdateTraits<'_> for PostgresqlTypeStdOptionOptionObjectAnimalToUpdatePostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionObjectAnimalWherePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeStdOptionOptionObjectAnimalWherePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionObjectAnimalWherePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfWhereTraits<'_> for PostgresqlTypeStdOptionOptionObjectAnimalWherePostgresqlJson {}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlType<'_> for StdOptionOptionObjectAnimalPostgresqlJson {
    type PostgresqlTypeSelf = StdOptionOptionObjectAnimalPostgresqlJson;
    type PostgresqlTypeSelfColumn = PostgresqlTypeStdOptionOptionObjectAnimalColumnPostgresqlJson;
    type PostgresqlTypeSelfToCreate = PostgresqlTypeStdOptionOptionObjectAnimalToCreatePostgresqlJson;
    type PostgresqlTypeSelfToRead = PostgresqlTypeStdOptionOptionObjectAnimalToReadPostgresqlJson;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeStdOptionOptionObjectAnimalToUpdatePostgresqlJson;
    type PostgresqlTypeSelfWhere = PostgresqlTypeStdOptionOptionObjectAnimalWherePostgresqlJson;
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdVecVecObjectWithIdAnimal(std::vec::Vec<ObjectWithIdAnimal>);
impl postgresql_crud::PostgresqlJsonType for StdVecVecObjectWithIdAnimal {
    type PostgresqlJsonTypeSelfToCreate<'a> = PostgresqlJsonTypeStdVecVecObjectWithIdAnimalToCreate;
    fn try_generate_postgresql_query_part_to_create(postgresql_json_type_self_to_create: &Self::PostgresqlJsonTypeSelfToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        let mut acc = std::string::String::default();
        for element in &postgresql_json_type_self_to_create.0 {
            match element.try_generate_postgresql_query_part_to_create(increment) {
                Ok(value) => {
                    acc.push_str(&format!("{value},"));
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _ = acc.pop();
        Ok(format!("jsonb_build_array({acc})"))
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(postgresql_json_type_self_to_create: Self::PostgresqlJsonTypeSelfToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in postgresql_json_type_self_to_create.0 {
            query = element.bind_value_to_postgresql_query_part_to_create(query);
        }
        query
    }
    type PostgresqlJsonTypeSelfFieldReader<'a> = PostgresqlJsonTypeStdVecVecObjectWithIdAnimalFieldReader;
    type PostgresqlJsonTypeSelfOptionsToRead<'a> = PostgresqlJsonTypeStdVecVecObjectWithIdAnimalOptionsToRead;
    fn generate_postgresql_query_part_to_read(
        postgresql_json_type_self_field_reader: &Self::PostgresqlJsonTypeSelfFieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
    ) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in &postgresql_json_type_self_field_reader.field_vec {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithId::Id(value) => <postgresql_crud::json_types::Uuid as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "id", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdPrimitiveI8(value) => <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdOptionOptionStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_option_option_std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdVecVecStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_vec_vec_std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithId::StdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_option_option_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        let start = postgresql_json_type_self_field_reader.pagination.start();
        let end = postgresql_json_type_self_field_reader.pagination.end();
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',(select jsonb_agg({acc}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end})))")
    }
    type PostgresqlJsonTypeSelfOptionToUpdate<'a> = PostgresqlJsonTypeStdVecVecObjectWithIdAnimalOptionToUpdate;
    type PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = PostgresqlJsonTypeStdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed;
    fn try_generate_postgresql_query_part_to_update(
        postgresql_json_type_self_option_to_update: &Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
        match postgresql_json_type_self_option_to_update.0.try_generate_postgresql_query_part_to_update(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment) {
            Ok(value) => Ok(value),
            Err(error) => {
                return Err(PostgresqlJsonTypeStdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::AnimalJsonArrayChangeTryGeneratePostgresqlQueryPartErrorNamed {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        }
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(
        postgresql_json_type_self_option_to_update: Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = postgresql_json_type_self_option_to_update.0.bind_value_to_postgresql_query_part_to_update(query);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdVecVecObjectWithIdAnimalPostgresqlJson;
impl std::fmt::Display for StdVecVecObjectWithIdAnimalPostgresqlJson {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for StdVecVecObjectWithIdAnimalPostgresqlJson {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl<'a> postgresql_crud::BindQuerySecond<'a> for StdVecVecObjectWithIdAnimalPostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::CreateTableQueryPart for StdVecVecObjectWithIdAnimalPostgresqlJson {
    fn create_table_query_part() -> impl std::fmt::Display {
        "JSONB"
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdVecVecObjectWithIdAnimalPostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ])
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToCreateTraits<'_> for StdVecVecObjectWithIdAnimalPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdVecVecObjectWithIdAnimalColumnPostgresqlJson;
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdVecVecObjectWithIdAnimalColumnPostgresqlJson {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        todo!()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdVecVecObjectWithIdAnimalColumnPostgresqlJson {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        todo!()
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdVecVecObjectWithIdAnimalColumnPostgresqlJson {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![]
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeStdVecVecObjectWithIdAnimalColumnPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdVecVecObjectWithIdAnimalToCreatePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeStdVecVecObjectWithIdAnimalToCreatePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdVecVecObjectWithIdAnimalToCreatePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToCreateTraits<'_> for PostgresqlTypeStdVecVecObjectWithIdAnimalToCreatePostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdVecVecObjectWithIdAnimalToReadPostgresqlJson;
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdVecVecObjectWithIdAnimalToReadPostgresqlJson {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        todo!()
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdVecVecObjectWithIdAnimalToReadPostgresqlJson {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeStdVecVecObjectWithIdAnimalToReadPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdVecVecObjectWithIdAnimalToUpdatePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeStdVecVecObjectWithIdAnimalToUpdatePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdVecVecObjectWithIdAnimalToUpdatePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToUpdateTraits<'_> for PostgresqlTypeStdVecVecObjectWithIdAnimalToUpdatePostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdVecVecObjectWithIdAnimalWherePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeStdVecVecObjectWithIdAnimalWherePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdVecVecObjectWithIdAnimalWherePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfWhereTraits<'_> for PostgresqlTypeStdVecVecObjectWithIdAnimalWherePostgresqlJson {}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlType<'_> for StdVecVecObjectWithIdAnimalPostgresqlJson {
    type PostgresqlTypeSelf = StdVecVecObjectWithIdAnimalPostgresqlJson;
    type PostgresqlTypeSelfColumn = PostgresqlTypeStdVecVecObjectWithIdAnimalColumnPostgresqlJson;
    type PostgresqlTypeSelfToCreate = PostgresqlTypeStdVecVecObjectWithIdAnimalToCreatePostgresqlJson;
    type PostgresqlTypeSelfToRead = PostgresqlTypeStdVecVecObjectWithIdAnimalToReadPostgresqlJson;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeStdVecVecObjectWithIdAnimalToUpdatePostgresqlJson;
    type PostgresqlTypeSelfWhere = PostgresqlTypeStdVecVecObjectWithIdAnimalWherePostgresqlJson;
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdOptionOptionStdVecVecObjectWithIdAnimal(std::option::Option<std::vec::Vec<ObjectWithIdAnimal>>);
impl postgresql_crud::PostgresqlJsonType for StdOptionOptionStdVecVecObjectWithIdAnimal {
    type PostgresqlJsonTypeSelfToCreate<'a> = PostgresqlJsonTypeStdOptionOptionStdVecVecObjectWithIdAnimalToCreate;
    fn try_generate_postgresql_query_part_to_create(postgresql_json_type_self_to_create: &Self::PostgresqlJsonTypeSelfToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
        match &postgresql_json_type_self_to_create.0 {
            Some(value) => {
                let mut acc = std::string::String::default();
                for element in value {
                    match element.try_generate_postgresql_query_part_to_create(increment) {
                        Ok(value) => {
                            acc.push_str(&format!("{value},"));
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                let _ = acc.pop();
                Ok(format!("jsonb_build_array({acc})"))
            }
            None => Ok(std::string::String::from("null")),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(postgresql_json_type_self_to_create: Self::PostgresqlJsonTypeSelfToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let Some(value) = postgresql_json_type_self_to_create.0 {
            for element in value {
                query = element.bind_value_to_postgresql_query_part_to_create(query);
            }
        }
        query
    }
    type PostgresqlJsonTypeSelfFieldReader<'a> = PostgresqlJsonTypeStdOptionOptionStdVecVecObjectWithIdAnimalFieldReader;
    type PostgresqlJsonTypeSelfOptionsToRead<'a> = PostgresqlJsonTypeStdOptionOptionStdVecVecObjectWithIdAnimalOptionsToRead;
    fn generate_postgresql_query_part_to_read(
        postgresql_json_type_self_field_reader: &Self::PostgresqlJsonTypeSelfFieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
    ) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in &postgresql_json_type_self_field_reader.field_vec {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithId::Id(value) => <postgresql_crud::json_types::Uuid as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "id", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdPrimitiveI8(value) => <postgresql_crud::json_types::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdOptionOptionStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_option_option_std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdVecVecStdPrimitiveI8(value) =>
                        <postgresql_crud::json_types::StdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(value, "std_vec_vec_std_primitive_i8", "value", &column_name_and_maybe_field_getter_for_error_message_field_ident),
                    AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithId::StdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_vec_vec_std_option_option_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                    AnimalFieldToReadWithId::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => <postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_query_part_to_read(
                        value,
                        "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
                        "value",
                        &column_name_and_maybe_field_getter_for_error_message_field_ident
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        let start = postgresql_json_type_self_field_reader.pagination.start();
        let end = postgresql_json_type_self_field_reader.pagination.end();
        format!
        ("jsonb_build_object('{field_ident}', jsonb_build_object('value', case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then null else (select jsonb_agg({acc}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end}) end))")
    }
    type PostgresqlJsonTypeSelfOptionToUpdate<'a> = PostgresqlJsonTypeStdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdate;
    type PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = PostgresqlJsonTypeStdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed;
    fn try_generate_postgresql_query_part_to_update(
        postgresql_json_type_self_option_to_update: &Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
        match &postgresql_json_type_self_option_to_update.0 {
            Some(value) => match value.try_generate_postgresql_query_part_to_update(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment) {
                Ok(value) => Ok(value),
                Err(error) => {
                    return Err(PostgresqlJsonTypeStdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::JsonArrayChange {
                        error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            },
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                }
                None => {
                    return Err(PostgresqlJsonTypeStdOptionOptionStdVecVecObjectWithIdAnimalOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                }
            },
        }
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(
        postgresql_json_type_self_option_to_update: Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match postgresql_json_type_self_option_to_update.0 {
            Some(value) => {
                query = value.bind_value_to_postgresql_query_part_to_update(query);
            }
            None => {
                query = query.bind(sqlx::types::Json(None::<std::option::Option<StdOptionOptionStdVecVecObjectWithIdAnimalJsonArrayChange>>));
            }
        }
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdOptionOptionStdVecVecObjectWithIdAnimalPostgresqlJson;
impl std::fmt::Display for StdOptionOptionStdVecVecObjectWithIdAnimalPostgresqlJson {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for StdOptionOptionStdVecVecObjectWithIdAnimalPostgresqlJson {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl<'a> postgresql_crud::BindQuerySecond<'a> for StdOptionOptionStdVecVecObjectWithIdAnimalPostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::CreateTableQueryPart for StdOptionOptionStdVecVecObjectWithIdAnimalPostgresqlJson {
    fn create_table_query_part() -> impl std::fmt::Display {
        "JSONB"
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdVecVecObjectWithIdAnimalPostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ]))
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToCreateTraits<'_> for StdOptionOptionStdVecVecObjectWithIdAnimalPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalColumnPostgresqlJson;
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalColumnPostgresqlJson {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        todo!()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalColumnPostgresqlJson {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        todo!()
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalColumnPostgresqlJson {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![]
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalColumnPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToCreatePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToCreatePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToCreatePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToCreateTraits<'_> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToCreatePostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToReadPostgresqlJson;
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToReadPostgresqlJson {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        todo!()
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToReadPostgresqlJson {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToReadPostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToUpdatePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToUpdatePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToUpdatePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfToUpdateTraits<'_> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToUpdatePostgresqlJson {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalWherePostgresqlJson;
impl<'a> postgresql_crud::BindQuerySecond<'a> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalWherePostgresqlJson {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalWherePostgresqlJson {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        todo!()
    }
}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlTypeSelfWhereTraits<'_> for PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalWherePostgresqlJson {}
impl postgresql_crud::postgresql_types::postgresql_type::PostgresqlType<'_> for StdOptionOptionStdVecVecObjectWithIdAnimalPostgresqlJson {
    type PostgresqlTypeSelf = StdOptionOptionStdVecVecObjectWithIdAnimalPostgresqlJson;
    type PostgresqlTypeSelfColumn = PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalColumnPostgresqlJson;
    type PostgresqlTypeSelfToCreate = PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToCreatePostgresqlJson;
    type PostgresqlTypeSelfToRead = PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToReadPostgresqlJson;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalToUpdatePostgresqlJson;
    type PostgresqlTypeSelfWhere = PostgresqlTypeStdOptionOptionStdVecVecObjectWithIdAnimalWherePostgresqlJson;
}
pub type AnimalToCreate = PostgresqlJsonTypeAnimalToCreate;
pub type AnimalOptionToUpdate = PostgresqlJsonTypeAnimalOptionToUpdate;
