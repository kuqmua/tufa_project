#[derive(Debug
    , postgresql_crud::GeneratePostgresqlCrud
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
    pub std_primitive_i16_as_small_int2_not_null: postgresql_crud::postgresql_type::StdPrimitiveI16AsInt2NotNull,
    pub std_primitive_i16_as_small_int2_nullable: postgresql_crud::postgresql_type::StdPrimitiveI16AsInt2Nullable,

    // pub std_primitive_i32_as_int4_not_null: postgresql_crud::postgresql_type::StdPrimitiveI32AsInt4NotNull,
    // pub std_primitive_i32_as_int4_nullable: postgresql_crud::postgresql_type::StdPrimitiveI32AsInt4Nullable,

    // pub std_primitive_i64_as_big_int8_not_null: postgresql_crud::postgresql_type::StdPrimitiveI64AsInt8NotNull,
    // pub std_primitive_i64_as_big_int8_nullable: postgresql_crud::postgresql_type::StdPrimitiveI64AsInt8Nullable,

    // pub std_primitive_f32_as_float4_not_null: postgresql_crud::postgresql_type::StdPrimitiveF32AsFloat4NotNull,
    // pub std_primitive_f32_as_float4_nullable: postgresql_crud::postgresql_type::StdPrimitiveF32AsFloat4Nullable,

    // pub std_primitive_f64_as_float8_not_null: postgresql_crud::postgresql_type::StdPrimitiveF64AsFloat8NotNull,
    // pub std_primitive_f64_as_float8_nullable: postgresql_crud::postgresql_type::StdPrimitiveF64AsFloat8Nullable,

    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i16_as_small_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::StdPrimitiveI16AsSmallSerialInitializedByPostgresqlNotNull,
    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i32_as_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::StdPrimitiveI32AsSerialInitializedByPostgresqlNotNull,
    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i64_as_big_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::StdPrimitiveI64AsBigSerialInitializedByPostgresqlNotNull,
    #[generate_postgresql_crud_primary_key]
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::postgresql_type::StdPrimitiveI64AsBigSerialInitializedByPostgresqlNotNull,

    // pub sqlx_postgres_types_pg_money_as_money_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsMoneyNotNull,
    // pub sqlx_postgres_types_pg_money_as_money_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsMoneyNullable,

    // pub sqlx_types_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::SqlxTypesDecimalAsNumericNotNull,
    // pub sqlx_types_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::SqlxTypesDecimalAsNumericNullable,

    // pub sqlx_types_big_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsNumericNotNull,
    // pub sqlx_types_big_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsNumericNullable,

    // pub std_primitive_bool_as_bool_not_null: postgresql_crud::postgresql_type::StdPrimitiveBoolAsBoolNotNull,
    // pub std_primitive_bool_as_bool_nullable: postgresql_crud::postgresql_type::StdPrimitiveBoolAsBoolNullable,

    // pub std_string_string_as_char_n_not_null: postgresql_crud::postgresql_type::StdStringStringAsCharNNotNull,
    // pub std_string_string_as_char_n_nullable: postgresql_crud::postgresql_type::StdStringStringAsCharNNullable,

    // pub std_string_string_as_varchar_not_null: postgresql_crud::postgresql_type::StdStringStringAsVarcharNotNull,
    // pub std_string_string_as_varchar_nullable: postgresql_crud::postgresql_type::StdStringStringAsVarcharNullable,

    // pub std_string_string_as_text_not_null: postgresql_crud::postgresql_type::StdStringStringAsTextNotNull,
    // pub std_string_string_as_text_nullable: postgresql_crud::postgresql_type::StdStringStringAsTextNullable,

    // // todo need to install postgresql extension
    // // pub std_string_string_as_ci_text_not_null: postgresql_crud::postgresql_type::StdStringStringAsCiTextNotNull,
    // // pub std_string_string_as_ci_text_nullable: postgresql_crud::postgresql_type::StdStringStringAsCiTextNullable,

    // // todo need to install postgresql extension
    // // pub sqlx_postgres_types_pg_ci_text_as_ci_text_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgCiTextAsCiTextNotNull,
    // // pub sqlx_postgres_types_pg_ci_text_as_ci_text_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgCiTextAsCiTextNullable,

    // pub std_vec_vec_std_primitive_u8_as_bytea_not_null: postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsByteaNotNull,
    // pub std_vec_vec_std_primitive_u8_as_bytea_nullable: postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsByteaNullable,

    // pub sqlx_types_chrono_naive_time_as_time_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsTimeNotNull,
    // pub sqlx_types_chrono_naive_time_as_time_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsTimeNullable,

    // pub sqlx_types_time_time_as_time_not_null: postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsTimeNotNull,
    // pub sqlx_types_time_time_as_time_nullable: postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsTimeNullable,

    // pub sqlx_postgres_types_pg_interval_as_interval_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsIntervalNotNull,
    // pub sqlx_postgres_types_pg_interval_as_interval_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsIntervalNullable,

    // pub sqlx_types_time_date_as_date_not_null: postgresql_crud::postgresql_type::SqlxTypesTimeDateAsDateNotNull,
    // pub sqlx_types_time_date_as_date_nullable: postgresql_crud::postgresql_type::SqlxTypesTimeDateAsDateNullable,

    // pub sqlx_types_chrono_naive_date_as_date_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsDateNotNull,
    // pub sqlx_types_chrono_naive_date_as_date_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsDateNullable,

    // pub chrono_naive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsTimestampNotNull,
    // pub chrono_naive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsTimestampNullable,

    // pub sqlx_types_time_primitive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsTimestampNotNull,
    // pub sqlx_types_time_primitive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsTimestampNullable,

    // pub chrono_date_time_chrono_utc_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzNotNull,
    // pub chrono_date_time_chrono_utc_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzNullable,

    // pub chrono_date_time_chrono_local_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzNotNull,
    // pub chrono_date_time_chrono_local_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzNullable,

    // pub uuid_uuid_as_uuid_v4_init_by_postgres_not_null: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresqlNotNull,
    // // #[generate_postgresql_crud_primary_key] //todo maybe later support something else but now i think uuid v4 and v7 is enough
    // // pub uuid_uuid_as_uuid_v4_init_by_postgres_not_null_prime: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresqlNotNull,

    // pub uuid_uuid_as_uuid_init_by_client_not_null: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsUuidInitializedByClientNotNull,
    // pub uuid_uuid_as_uuid_init_by_client_nullable: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsUuidInitializedByClientNullable,

    // pub sqlx_types_ipnetwork_ip_network_as_inet_not_null: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsInetNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_inet_nullable: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsInetNullable,

    // pub sqlx_types_ipnetwork_ip_network_as_cidr_not_null: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsCidrNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_cidr_nullable: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsCidrNullable,

    // pub mac_address_mac_address_as_mac_addr_not_null: postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsMacAddrNotNull,
    // pub mac_address_mac_address_as_mac_addr_nullable: postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsMacAddrNullable,

    // pub sqlx_types_bit_vec_as_bit_not_null: postgresql_crud::postgresql_type::SqlxTypesBitVecAsBitNotNull,
    // pub sqlx_types_bit_vec_as_bit_nullable: postgresql_crud::postgresql_type::SqlxTypesBitVecAsBitNullable,

    // pub sqlx_types_bit_vec_as_varbit_not_null: postgresql_crud::postgresql_type::SqlxTypesBitVecAsVarbitNotNull,
    // pub sqlx_types_bit_vec_as_varbit_nullable: postgresql_crud::postgresql_type::SqlxTypesBitVecAsVarbitNullable,

    // pub pg_range_std_primitive_i32_as_int4_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4RangeNotNull,
    // pub pg_range_std_primitive_i32_as_int4_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4RangeNullable,

    // pub pg_range_std_primitive_i64_as_int8_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8RangeNotNull,
    // pub pg_range_std_primitive_i64_as_int8_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8RangeNullable,

    // pub pg_range_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsNumRangeNotNull,
    // pub pg_range_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsNumRangeNullable,

    // pub pg_range_sqlx_types_big_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRangeNotNull,
    // pub pg_range_sqlx_types_big_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRangeNullable,

    // pub pg_range_sqlx_types_time_date_as_date_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRangeNotNull,
    // pub pg_range_sqlx_types_time_date_as_date_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRangeNullable,

    // pub pg_range_chrono_naive_date_as_date_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRangeNotNull,
    // pub pg_range_chrono_naive_date_as_date_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRangeNullable,

    // pub pg_range_naive_date_time_as_timestamp_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRangeNotNull,
    // pub pg_range_naive_date_time_as_timestamp_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRangeNullable,

    // pub pg_range_time_primitive_date_time_as_timestamp_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRangeNotNull,
    // pub pg_range_time_primitive_date_time_as_timestamp_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRangeNullable,

    // pub pg_range_date_time_utc_as_timestamp_tz_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRangeNotNull,
    // pub pg_range_date_time_utc_as_timestamp_tz_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRangeNullable,

    // pub pg_range_date_time_local_as_timestamp_tz_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRangeNotNull,
    // pub pg_range_date_time_local_as_timestamp_tz_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRangeNullable,
    pub object_animal_as_jsonb_not_null: ObjectAnimalAsJsonbNotNull,
    // pub object_animal_as_jsonb_nullable: ObjectAnimalAsJsonbNullable,

    // pub option_object_animal_as_jsonb_not_null: StdOptionOptionObjectAnimalAsJsonbNotNull,
    // pub option_object_animal_as_jsonb_nullable: StdOptionOptionObjectAnimalAsJsonbNullable,

    // pub std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdVecVecObjectWithIdAnimalAsJsonbNotNull,
    // pub std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdVecVecObjectWithIdAnimalAsJsonbNullable,

    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdOptionOptionStdVecVecObjectWithIdAnimalAsJsonbNotNull,
    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdOptionOptionStdVecVecObjectWithIdAnimalAsJsonbNullable,

    ///////////////////////////
    // pub vec_std_primitive_i16_as_postgresql_int2_array_not_null: postgresql_crud::postgresql_type::VecStdPrimitiveI16AsInt2ArrayNotNull,
    //63max
}
//todo enum tree support
//todo generate wrapper type for all possible json type
#[derive(Debug, postgresql_crud::GeneratePostgresqlJsonType)]
pub struct Animal {
    // pub id: postgresql_crud::postgresql_json_type::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    // pub std_vec_vec_uuid_uuid: postgresql_crud::postgresql_json_type::StdVecVecUuidUuid,
    // pub std_vec_vec_std_vec_vec_uuid_uuid: postgresql_crud::postgresql_json_type::StdVecVecStdVecVecUuidUuid,
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
    // pub std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI8,
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
///////////////////////////////////////////////////
pub async fn create_table_if_not_exists(pool: &sqlx::Pool<sqlx::Postgres>) {
    let create_extension_if_not_exists_pg_jsonschema_query_stringified = "create extension if not exists pg_jsonschema";
    println!("{create_extension_if_not_exists_pg_jsonschema_query_stringified}");
    let _ = sqlx::query(create_extension_if_not_exists_pg_jsonschema_query_stringified).execute(pool).await.unwrap();
    let create_extension_if_not_exists_uuid_ossp_query_stringified = "create extension if not exists \"uuid-ossp\"";
    println!("{create_extension_if_not_exists_uuid_ossp_query_stringified}");
    let _ = sqlx::query(create_extension_if_not_exists_uuid_ossp_query_stringified).execute(pool).await.unwrap();
    let create_table_if_not_exists_query_stringified = format!(
        // "create table if not exists example ({},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        "create table if not exists example ({},{},{},{})",
        // "create table if not exists example ({},{},{})",
        <postgresql_crud::postgresql_type::StdPrimitiveI64AsBigSerialInitializedByPostgresqlNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_i64_as_postgresql_big_serial_not_null_primary_key", true),
        <postgresql_crud::postgresql_type::StdPrimitiveI16AsInt2NotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_i16_as_small_int2_not_null", false),
        <postgresql_crud::postgresql_type::StdPrimitiveI16AsInt2Nullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_i16_as_small_int2_nullable", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveI32AsInt4NotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_i32_as_int4_not_null", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveI32AsInt4Nullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_i32_as_int4_nullable", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveI64AsInt8NotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_i64_as_big_int8_not_null", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveI64AsInt8Nullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_i64_as_big_int8_nullable", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveF32AsFloat4NotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_f32_as_float4_not_null", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveF32AsFloat4Nullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_f32_as_float4_nullable", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveF64AsFloat8NotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_f64_as_float8_not_null", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveF64AsFloat8Nullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_f64_as_float8_nullable", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveI16AsSmallSerialInitializedByPostgresqlNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_i16_as_small_serial_init_by_postgres_not_null", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveI32AsSerialInitializedByPostgresqlNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_i32_as_serial_init_by_postgres_not_null", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveI64AsBigSerialInitializedByPostgresqlNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_i64_as_big_serial_init_by_postgres_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsMoneyNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_postgres_types_pg_money_as_money_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsMoneyNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_postgres_types_pg_money_as_money_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesDecimalAsNumericNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_decimal_as_numeric_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesDecimalAsNumericNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_decimal_as_numeric_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsNumericNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_big_decimal_as_numeric_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsNumericNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_big_decimal_as_numeric_nullable", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveBoolAsBoolNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_bool_as_bool_not_null", false),
        // <postgresql_crud::postgresql_type::StdPrimitiveBoolAsBoolNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_primitive_bool_as_bool_nullable", false),
        // <postgresql_crud::postgresql_type::StdStringStringAsCharNNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_string_string_as_char_n_not_null", false, postgresql_crud::postgresql_type::StdStringStringAsCharNLength::try_from(1).unwrap()),
        // <postgresql_crud::postgresql_type::StdStringStringAsCharNNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_string_string_as_char_n_nullable", false, postgresql_crud::postgresql_type::StdStringStringAsCharNLength::try_from(1).unwrap()),
        // <postgresql_crud::postgresql_type::StdStringStringAsVarcharNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_string_string_as_varchar_not_null", false, postgresql_crud::postgresql_type::StdStringStringAsVarcharLength::try_from(1).unwrap()),
        // <postgresql_crud::postgresql_type::StdStringStringAsVarcharNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_string_string_as_varchar_nullable", false, postgresql_crud::postgresql_type::StdStringStringAsVarcharLength::try_from(1).unwrap()),
        // <postgresql_crud::postgresql_type::StdStringStringAsTextNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_string_string_as_text_not_null", false),
        // <postgresql_crud::postgresql_type::StdStringStringAsTextNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_string_string_as_text_nullable", false),
        // <postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsByteaNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_vec_vec_std_primitive_u8_as_bytea_not_null", false),
        // <postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsByteaNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"std_vec_vec_std_primitive_u8_as_bytea_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesTimeDateAsDateNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_time_date_as_date_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesTimeDateAsDateNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_time_date_as_date_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsDateNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_chrono_naive_date_as_date_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsDateNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_chrono_naive_date_as_date_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsTimeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_chrono_naive_time_as_time_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsTimeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_chrono_naive_time_as_time_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsTimeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_time_time_as_time_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsTimeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_time_time_as_time_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsIntervalNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_postgres_types_pg_interval_as_interval_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsIntervalNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_postgres_types_pg_interval_as_interval_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsTimestampNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"chrono_naive_date_time_as_timestamp_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsTimestampNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"chrono_naive_date_time_as_timestamp_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsTimestampNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_time_primitive_date_time_as_timestamp_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsTimestampNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_time_primitive_date_time_as_timestamp_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"chrono_date_time_chrono_utc_as_timestamp_tz_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"chrono_date_time_chrono_utc_as_timestamp_tz_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"chrono_date_time_chrono_local_as_timestamp_tz_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"chrono_date_time_chrono_local_as_timestamp_tz_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresqlNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"uuid_uuid_as_uuid_v4_init_by_postgres_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsUuidInitializedByClientNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"uuid_uuid_as_uuid_init_by_client_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsUuidInitializedByClientNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"uuid_uuid_as_uuid_init_by_client_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsInetNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_inet_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsInetNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_inet_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsCidrNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_cidr_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsCidrNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_cidr_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsMacAddrNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"mac_address_mac_address_as_mac_addr_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsMacAddrNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"mac_address_mac_address_as_mac_addr_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxTypesBitVecAsBitNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_bit_vec_as_bit_not_null", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsBitLength::try_from(1).unwrap()),
        // <postgresql_crud::postgresql_type::SqlxTypesBitVecAsBitNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_bit_vec_as_bit_nullable", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsBitLength::try_from(1).unwrap()),
        // <postgresql_crud::postgresql_type::SqlxTypesBitVecAsVarbitNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_bit_vec_as_varbit_not_null", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsVarbitLength::try_from(1).unwrap()),
        // <postgresql_crud::postgresql_type::SqlxTypesBitVecAsVarbitNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"sqlx_types_bit_vec_as_varbit_nullable", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsVarbitLength::try_from(1).unwrap()),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4RangeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_std_primitive_i32_as_int4_range_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4RangeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_std_primitive_i32_as_int4_range_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8RangeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_std_primitive_i64_as_int8_range_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8RangeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_std_primitive_i64_as_int8_range_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRangeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_naive_date_time_as_timestamp_range_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRangeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_naive_date_time_as_timestamp_range_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRangeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_time_primitive_date_time_as_timestamp_range_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRangeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_time_primitive_date_time_as_timestamp_range_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRangeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_date_time_utc_as_timestamp_tz_range_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRangeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_date_time_utc_as_timestamp_tz_range_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRangeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_date_time_local_as_timestamp_tz_range_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRangeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_date_time_local_as_timestamp_tz_range_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRangeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_chrono_naive_date_as_date_range_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRangeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_chrono_naive_date_as_date_range_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRangeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_sqlx_types_time_date_as_date_range_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRangeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_sqlx_types_time_date_as_date_range_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsNumRangeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_decimal_as_num_range_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsNumRangeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_decimal_as_num_range_nullable", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRangeNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_sqlx_types_big_decimal_as_num_range_not_null", false),
        // <postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRangeNullable as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"pg_range_sqlx_types_big_decimal_as_num_range_nullable", false),
        <ObjectAnimalAsJsonbNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"object_animal_as_jsonb_not_null", false),
        // <postgresql_crud::postgresql_type::VecStdPrimitiveI16AsInt2ArrayNotNull as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"vec_std_primitive_i16_as_postgresql_int2_array_not_null", false),
    );
    println!("{create_table_if_not_exists_query_stringified}");
    let _ = sqlx::query(&create_table_if_not_exists_query_stringified).execute(pool).await.unwrap();
}

//////////////////////////////////
#[cfg(test)]
mod tests {
    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<crate::repositories_types::server::routes::api::example::Example>(), 0);
    }
}
//////////////////
