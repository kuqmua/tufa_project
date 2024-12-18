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
    // pub std_primitive_bool_as_postgresql_bool: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBool,
    pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    // pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlSmallInt,
    // pub std_primitive_i16_as_postgresql_small_int_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    // pub std_primitive_i16_as_postgresql_small_serial: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerial,
    // pub std_primitive_i16_as_postgresql_small_serial_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    // pub std_primitive_i16_as_postgresql_small_int2: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2,
    // pub std_primitive_i16_as_postgresql_small_int2_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull,

    // pub std_primitive_i32_as_postgresql_int: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt,
    // pub std_primitive_i32_as_postgresql_int_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlIntNotNull,
    // pub std_primitive_i32_as_postgresql_serial: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlSerial,
    // pub std_primitive_i32_as_postgresql_serial_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlSerialNotNull,
    // pub std_primitive_i32_as_postgresql_int4: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt4,
    // pub std_primitive_i32_as_postgresql_int4_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt4NotNull,

    // pub std_primitive_i64_as_postgresql_big_int: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigInt,
    // pub std_primitive_i64_as_postgresql_big_int_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigIntNotNull,
    // pub std_primitive_i64_as_postgresql_big_serial: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerial,
    #[generate_postgresql_crud_second_primary_key]
    pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    // pub std_primitive_i64_as_postgresql_big_int8: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlInt8,
    // pub std_primitive_i64_as_postgresql_big_int8_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlInt8NotNull,

    // pub std_primitive_f32_as_postgresql_real: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlReal,
    // pub std_primitive_f32_as_postgresql_real_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlRealNotNull,
    // pub std_primitive_f32_as_postgresql_float4: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4,
    // pub std_primitive_f32_as_postgresql_float4_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4NotNull,

    // pub std_primitive_f64_as_postgresql_double_precision: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlDoublePrecision,
    // pub std_primitive_f64_as_postgresql_double_precision_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull,
    // pub std_primitive_f64_as_postgresql_float8: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8,
    // pub std_primitive_f64_as_postgresql_float8_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8NotNull,

    // pub std_string_string_as_postgresql_varchar: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarchar,
    // pub std_string_string_as_postgresql_varchar_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharNotNull,
    // // pub std_string_string_as_postgresql_char_n: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharN,
    // // pub std_string_string_as_postgresql_char_n_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNNotNull,
    // pub std_string_string_as_postgresql_text: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlText,
    // pub std_string_string_as_postgresql_text_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlTextNotNull,
    // // pub std_string_string_as_postgresql_ci_text: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCiText,
    // // pub std_string_string_as_postgresql_ci_text_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCiTextNotNull,

    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea: postgresql_crud::postgresql_type::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlBytea,
    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea_not_null: postgresql_crud::postgresql_type::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_money_as_postgresql_money: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
    // pub sqlx_postgres_types_pg_money_as_postgresql_money_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiText,
    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

    // pub sqlx_types_big_decimal_as_postgresql_numeric: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumeric,
    // pub sqlx_types_big_decimal_as_postgresql_numeric_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_decimal_as_postgresql_numeric: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesDecimalAsPostgresqlNumeric,
    // pub sqlx_types_decimal_as_postgresql_numeric_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_chrono_naive_date_as_postgresql_date: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDate,
    // pub sqlx_types_chrono_naive_date_as_postgresql_date_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

    // pub sqlx_types_chrono_naive_time_as_postgresql_time: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTime,
    // pub sqlx_types_chrono_naive_time_as_postgresql_time_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,
    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull,

    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,
    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_time_date_as_postgresql_date: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeDateAsPostgresqlDate,
    // pub sqlx_types_time_date_as_postgresql_date_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNotNull,

    // pub sqlx_types_time_time_as_postgresql_time: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTime,
    // pub sqlx_types_time_time_as_postgresql_time_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    // pub sqlx_types_uuid_uuid_as_postgresql_uuid: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuid,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidNotNull,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough //fails too but primary key is a different logic. need refactor it as different task

    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    // pub std_net_ip_addr_as_postgresql_inet: postgresql_crud::postgresql_type::postgresql_type::StdNetIpAddrAsPostgresqlInet,
    // pub std_net_ip_addr_as_postgresql_inet_not_null: postgresql_crud::postgresql_type::postgresql_type::StdNetIpAddrAsPostgresqlInetNotNull,
    // pub std_net_ip_addr_as_postgresql_cidr: postgresql_crud::postgresql_type::postgresql_type::StdNetIpAddrAsPostgresqlCidr,
    // pub std_net_ip_addr_as_postgresql_cidr_not_null: postgresql_crud::postgresql_type::postgresql_type::StdNetIpAddrAsPostgresqlCidrNotNull,

    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

    // pub sqlx_types_bit_vec_as_postgresql_bit: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBit,
    // pub sqlx_types_bit_vec_as_postgresql_bit_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNotNull,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarBit,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarBitNotNull,

    // pub object_animal_as_postgres: ObjectAnimalAsPostgresqlJsonbNotNull,
    // pub std_option_option_object_animal_as_postgres: StdOptionOptionObjectAnimalAsPostgresqlJsonbNotNull,
    // pub std_vec_vec_object_with_id_animal_as_postgres: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,
    // pub std_option_option_std_vec_vec_object_with_id_animal_as_postgres: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,//63max

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
    postgresql_crud::GeneratePostgresqlJsonType
)] //user type must implement utoipa::ToSchema trait
pub struct Animal {
    // pub id: postgresql_crud::postgresql_json_type::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    //todo check field max length in postgresql
    pub std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8,
    // pub std_primitive_i16: postgresql_crud::postgresql_json_type::StdPrimitiveI16,
    // pub std_primitive_i32: postgresql_crud::postgresql_json_type::StdPrimitiveI32,
    // pub std_primitive_i64: postgresql_crud::postgresql_json_type::StdPrimitiveI64,
    // pub std_primitive_u8: postgresql_crud::postgresql_json_type::StdPrimitiveU8,
    // pub std_primitive_u16: postgresql_crud::postgresql_json_type::StdPrimitiveU16,
    // pub std_primitive_u32: postgresql_crud::postgresql_json_type::StdPrimitiveU32,
    // pub std_primitive_u64: postgresql_crud::postgresql_json_type::StdPrimitiveU64,
    // pub std_primitive_f32: postgresql_crud::postgresql_json_type::StdPrimitiveF32,
    // pub std_primitive_f64: postgresql_crud::postgresql_json_type::StdPrimitiveF64,
    // pub std_primitive_bool: postgresql_crud::postgresql_json_type::StdPrimitiveBool,
    // pub std_string_string: postgresql_crud::postgresql_json_type::StdStringString,
    pub std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI8,
    // pub std_option_option_std_primitive_i16: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_primitive_i32: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_primitive_i64: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_primitive_u8: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_primitive_u16: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_primitive_u32: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_primitive_u64: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_primitive_f32: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_primitive_f64: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_primitive_bool: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_string_string: postgresql_crud::postgresql_json_type::StdOptionOptionStdStringString,
    // pub std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI8,
    // pub std_vec_vec_std_primitive_i16: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI16,
    // pub std_vec_vec_std_primitive_i32: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI32,
    // pub std_vec_vec_std_primitive_i64: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI64,
    // pub std_vec_vec_std_primitive_u8: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU8,
    // pub std_vec_vec_std_primitive_u16: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU16,
    // pub std_vec_vec_std_primitive_u32: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU32,
    // pub std_vec_vec_std_primitive_u64: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU64,
    // pub std_vec_vec_std_primitive_f32: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveF32,
    // pub std_vec_vec_std_primitive_f64: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveF64,
    // pub std_vec_vec_std_primitive_bool: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveBool,
    // pub std_vec_vec_std_string_string: postgresql_crud::postgresql_json_type::StdVecVecStdStringString,
    // pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI8,
    // pub std_option_option_std_vec_vec_std_primitive_i16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_primitive_i32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_primitive_i64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_primitive_u8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_primitive_u16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_primitive_u32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_primitive_u64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_primitive_f32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_primitive_f64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_primitive_bool: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_string_string: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdStringString,
    // pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI8,
    // pub std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_vec_vec_std_option_option_std_string_string: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdStringString,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_option_option_std_string_string: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdStringString,

    // pub object_doggie: ObjectDoggie,
    // pub std_option_option_object: StdOptionOptionObjectDoggie,
    // pub std_vec_vec_object: StdVecVecObjectWithIdDoggie,
    // pub std_option_option_std_vec_vec_object: StdOptionOptionStdVecVecObjectWithIdDoggie,

    // pub std_vec_vec_object_with_id: StdVecVecObjectWithIdDoggie,
    // pub std_option_option_std_vec_vec_object_with_id: StdOptionOptionStdVecVecObjectWithIdDoggie
}
// /////////////////////////////////////////
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud::GeneratePostgresqlJsonType
// )] //user type must implement utoipa::ToSchema trait
// pub struct Doggie {
//     pub std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8,
//     pub std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI8,
//     pub std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI8,
//     pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
// }
/////////////////////////////////
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadManyPayload {
    pub std_primitive_bool_as_postgresql_bool_not_null: 
    // std::option::Option<std::vec::Vec<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhere>>,
    std::option::Option<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhere>,
    pub std_primitive_i64_as_postgresql_big_serial_not_null: 
    // std::option::Option<std::vec::Vec<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere>>,
    std::option::Option<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere>,
    pub select: std::vec::Vec<PostgresqlTypeExampleColumn>,
    pub order_by: postgresql_crud::OrderBy<PostgresqlTypeExampleColumn>,
    pub limit: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    pub offset: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
}
#[derive(Debug)]
pub struct ReadManyParameters {
    pub payload: ReadManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<ExampleOptions>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        not_unique_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        not_unique_column: PostgresqlTypeExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        std_primitive_bool_as_postgresql_bool_not_null: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyRouteLogicErrorNamed> for TryReadManyRouteLogicResponseVariants {
    fn from(value: TryReadManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => Self::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence } => Self::NotUniqueColumn { not_unique_column, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                std_primitive_bool_as_postgresql_bool_not_null,
                code_occurence,
            } => Self::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                std_primitive_bool_as_postgresql_bool_not_null,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: PostgresqlTypeExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        #[eo_to_std_string_string]
        std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2561,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = ReadManyParameters {
        payload: match serde_json::from_slice::<ReadManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = ReadManyPayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryReadManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2633,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = {
        format!(
            "select {} from example where {}",
            {
                let mut value = std::string::String::default();
                for element in &parameters.payload.select {
                    value.push_str(&match element {
                        PostgresqlTypeExampleColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull(value) => {
                            <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::postgresql_type_self_column_query_part(value, "std_primitive_bool_as_postgresql_bool_not_null")
                        }
                        PostgresqlTypeExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNull(value) => {
                            <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::postgresql_type_self_column_query_part(
                                value,
                                "std_primitive_i64_as_postgresql_big_serial_not_null",
                            )
                        }
                    });
                    value.push_str(",");
                }
                let _ = value.pop();
                value
            },
            {
                let mut increment: std::primitive::u64 = 0;
                let mut additional_parameters = std::string::String::default();
                if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
                    // additional_parameters.push_str(&format!(
                    //     "{} {}",
                    //     match additional_parameters.is_empty() {
                    //         true => "where",
                    //         false => " and",
                    //     },
                    //     {
                    //         let mut acc = std::string::String::default();
                    //         for (index, element) in value.iter().enumerate() {
                    //             match postgresql_crud::BindQuerySecond::try_generate_bind_increments(element, &mut increment) {
                    //                 Ok(value) => {
                    //                     let handle = format!("std_primitive_bool_as_postgresql_bool_not_null {value} ");
                    //                     match index == 0 {
                    //                         true => {
                    //                             acc.push_str(&handle);
                    //                         }
                    //                         false => {
                    //                             acc.push_str(&format!("{} {handle}", element.conjunctive_operator));
                    //                         }
                    //                     }
                    //                 }
                    //                 Err(error_0) => {
                    //                     let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                    //                         bind_query: error_0,
                    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    //                             file!().to_owned(),
                    //                             line!(),
                    //                             column!(),
                    //                             Some(error_occurence_lib::code_occurence::MacroOccurence {
                    //                                 file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                    //                                 line: 3625,
                    //                                 column: 266,
                    //                             }),
                    //                         ),
                    //                     };
                    //                     eprintln!("{error}");
                    //                     let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                    //                     *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    //                     return response;
                    //                 }
                    //             }
                    //         }
                    //         if let false = acc.is_empty() {
                    //             let _ = acc.pop();
                    //         }
                    //         acc
                    //     }
                    // ));
                    match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::postgresql_type_self_where_try_generate_bind_increments(
                        &value,
                        &mut increment,
                        &"std_primitive_bool_as_postgresql_bool_not_null",
                        true,//todo generate is in proc macro (first element ignore)
                    ) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                        }
                        Err(error_0) => {
                            // let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                            //     bind_query: error_0,
                            //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            //         file!().to_owned(),
                            //         line!(),
                            //         column!(),
                            //         Some(error_occurence_lib::code_occurence::MacroOccurence {
                            //             file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            //             line: 3625,
                            //             column: 266,
                            //         }),
                            //     ),
                            // };
                            // eprintln!("{error}");
                            // let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            // *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            // return response;
                            todo!()
                        }
                    }
                }
                if let Some(value) = &parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null {
                    // additional_parameters.push_str(&format!(
                    //     "{} {}",
                    //     match additional_parameters.is_empty() {
                    //         true => "where",
                    //         false => " and",
                    //     },
                    //     {
                    //         let mut acc = std::string::String::default();
                    //         for (index, element) in value.iter().enumerate() {
                    //             match postgresql_crud::BindQuerySecond::try_generate_bind_increments(element, &mut increment) {
                    //                 Ok(value) => {
                    //                     let handle = format!("std_primitive_i64_as_postgresql_big_serial_not_null {value} ");
                    //                     match index == 0 {
                    //                         true => {
                    //                             acc.push_str(&handle);
                    //                         }
                    //                         false => {
                    //                             acc.push_str(&format!("{} {handle}", element.conjunctive_operator));
                    //                         }
                    //                     }
                    //                 }
                    //                 Err(error_0) => {
                    //                     let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                    //                         bind_query: error_0,
                    //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    //                             file!().to_owned(),
                    //                             line!(),
                    //                             column!(),
                    //                             Some(error_occurence_lib::code_occurence::MacroOccurence {
                    //                                 file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                    //                                 line: 3625,
                    //                                 column: 266,
                    //                             }),
                    //                         ),
                    //                     };
                    //                     eprintln!("{error}");
                    //                     let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                    //                     *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    //                     return response;
                    //                 }
                    //             }
                    //         }
                    //         if let false = acc.is_empty() {
                    //             let _ = acc.pop();
                    //         }
                    //         acc
                    //     }
                    // ));
                    match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::postgresql_type_self_where_try_generate_bind_increments(
                        &value,
                        &mut increment,
                        &"std_primitive_i64_as_postgresql_big_serial_not_null",
                        false,//todo generate is in proc macro (first element ignore)
                    ) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                        }
                        Err(error_0) => {
                            // let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                            //     bind_query: error_0,
                            //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            //         file!().to_owned(),
                            //         line!(),
                            //         column!(),
                            //         Some(error_occurence_lib::code_occurence::MacroOccurence {
                            //             file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            //             line: 3625,
                            //             column: 266,
                            //         }),
                            //     ),
                            // };
                            // eprintln!("{error}");
                            // let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            // *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            // return response;
                            todo!()
                        }
                    }
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = &parameters.payload.order_by;
                    let order = match &value.order {
                        Some(value) => value.to_snake_case_stringified(),
                        None => postgresql_crud::Order::default().to_snake_case_stringified(),
                    };
                    additional_parameters.push_str(&format!("{}order by {} {}", prefix, value.column.pick_column(), order,));
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&parameters.payload.limit, &mut increment) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                bind_query: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 3673,
                                        column: 168,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    };
                    additional_parameters.push_str(&format!("{}limit {}", prefix, value));
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&parameters.payload.offset, &mut increment) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                bind_query: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 3675,
                                        column: 168,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    };
                    additional_parameters.push_str(&format!("{}offset {}", prefix, value));
                }
                additional_parameters
            }
        )
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
            // for value in value {
            //     query = postgresql_crud::BindQuerySecond::bind_value_to_query(value, query);
            // }
        }
        if let Some(value) = parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null {
            // for value in value {
            //     query = postgresql_crud::BindQuerySecond::bind_value_to_query(value, query);
            // }
        }
        query = postgresql_crud::BindQuerySecond::bind_value_to_query(parameters.payload.limit, query);
        query = postgresql_crud::BindQuerySecond::bind_value_to_query(parameters.payload.offset, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2583,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2583,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let value = {
            let mut rows = binded_query.fetch(executor.as_mut());
            let mut acc = std::vec::Vec::new();
            while let Some(value) = match {
                use postgresql_crud::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => match value {
                    Some(value) => Some({
                        let mut std_primitive_i64_as_postgresql_big_serial_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToRead>> = None;
                        let mut std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToRead>> = None;
                        for element in &parameters.payload.select {
                            match element {
                                PostgresqlTypeExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNull(_) => {
                                    match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToRead, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null") {
                                        Ok(value) => {
                                            std_primitive_i64_as_postgresql_big_serial_not_null = Some(postgresql_crud::Value { value: value });
                                        }
                                        Err(error_0) => {
                                            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                                postgresql: error_0,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!(),
                                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                        line: 1248,
                                                        column: 246,
                                                    }),
                                                ),
                                            };
                                            eprintln!("{error}");
                                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                            return response;
                                        }
                                    }
                                }
                                PostgresqlTypeExampleColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull(_) => {
                                    match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToRead, &std::primitive::str>(&value, "std_primitive_bool_as_postgresql_bool_not_null") {
                                        Ok(value) => {
                                            std_primitive_bool_as_postgresql_bool_not_null = Some(postgresql_crud::Value { value: value });
                                        }
                                        Err(error_0) => {
                                            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                                postgresql: error_0,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!(),
                                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                        line: 1283,
                                                        column: 246,
                                                    }),
                                                ),
                                            };
                                            eprintln!("{error}");
                                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                            return response;
                                        }
                                    }
                                }
                            }
                        }
                        ExampleOptions {
                            std_primitive_bool_as_postgresql_bool_not_null,
                            std_primitive_i64_as_postgresql_big_serial_not_null,
                        }
                    }),
                    None => None,
                },
                Err(error_0) => {
                    let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 3784,
                                column: 169,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            } {
                acc.push(value);
            }
            acc
        };
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        #[eo_to_std_string_string]
        std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: PostgresqlTypeExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_many_route_logic_error_named_with_serialize_deserialize: TryReadManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many(server_location: &std::primitive::str, parameters: ReadManyParameters) -> Result<std::vec::Vec<ExampleOptions>, TryReadManyErrorNamed> {
    let payload = {
        let value = ReadManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryReadManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2685,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/read_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryReadManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2737,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryReadManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2754,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryReadManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryReadManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2767,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_read_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryReadManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value.into_iter().fold(std::vec::Vec::new(), |mut acc, element| {
                acc.push(element);
                acc
            });
            return Ok(value);
        }
        TryReadManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryReadManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryReadManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryReadManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryReadManyRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
        TryReadManyRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniqueColumn { not_unique_column, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
            std_primitive_bool_as_postgresql_bool_not_null,
            code_occurence,
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
            std_primitive_bool_as_postgresql_bool_not_null,
            code_occurence,
        },
    };
    Err(TryReadManyErrorNamed::TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_read_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                line: 2805,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ReadManyPayload {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_bool_as_postgresql_bool_not_null: Some(
                postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ),
            std_primitive_i64_as_postgresql_big_serial_not_null: Some(
                postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ),
            select: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            order_by: postgresql_crud::OrderBy {
                column: PostgresqlTypeExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNull(
                    postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                ),
                order: Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            },
            limit: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            offset: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn read_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <ReadManyPayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
