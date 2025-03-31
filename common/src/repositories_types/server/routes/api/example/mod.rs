#[derive(
    Debug, 
    postgresql_crud::GeneratePostgresqlCrud
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
    pub std_primitive_i16_as_small_int2_not_null: postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull,
    pub std_primitive_i16_as_small_int2_nullable: postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2Nullable,

    // pub std_primitive_i32_as_int4_not_null: postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlInt4NotNull,
    // pub std_primitive_i32_as_int4_nullable: postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlInt4Nullable,

    // pub std_primitive_i64_as_big_int8_not_null: postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlInt8NotNull,
    // pub std_primitive_i64_as_big_int8_nullable: postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlInt8Nullable,

    // pub std_primitive_f32_as_float4_not_null: postgresql_crud::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4NotNull,
    // pub std_primitive_f32_as_float4_nullable: postgresql_crud::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4Nullable,

    // pub std_primitive_f64_as_float8_not_null: postgresql_crud::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8NotNull,
    // pub std_primitive_f64_as_float8_nullable: postgresql_crud::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8Nullable,

    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i16_as_small_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresqlNotNull,
    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i32_as_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresqlNotNull,
    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i64_as_big_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,
    #[generate_postgresql_crud_primary_key]
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,

    // pub sqlx_postgres_types_pg_money_as_money_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,
    // pub sqlx_postgres_types_pg_money_as_money_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNullable,

    // pub sqlx_types_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNotNull,
    // pub sqlx_types_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNullable,

    // pub sqlx_types_big_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,
    // pub sqlx_types_big_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNullable,

    // pub std_primitive_bool_as_bool_not_null: postgresql_crud::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull,
    // pub std_primitive_bool_as_bool_nullable: postgresql_crud::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNullable,

    // pub std_string_string_as_char_n_not_null: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNNotNull,
    // pub std_string_string_as_char_n_nullable: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNNullable,

    // pub std_string_string_as_varchar_not_null: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharNotNull,
    // pub std_string_string_as_varchar_nullable: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharNullable,

    // pub std_string_string_as_text_not_null: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlTextNotNull,
    // pub std_string_string_as_text_nullable: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlTextNullable,

    // // todo need to install postgresql extension
    // // pub std_string_string_as_ci_text_not_null: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCiTextNotNull,
    // // pub std_string_string_as_ci_text_nullable: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCiTextNullable,

    // // todo need to install postgresql extension
    // // pub sqlx_postgres_types_pg_ci_text_as_ci_text_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,
    // // pub sqlx_postgres_types_pg_ci_text_as_ci_text_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNullable,

    // pub std_vec_vec_std_primitive_u8_as_bytea_not_null: postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,
    // pub std_vec_vec_std_primitive_u8_as_bytea_nullable: postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNullable,

    // pub sqlx_types_chrono_naive_time_as_time_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,
    // pub sqlx_types_chrono_naive_time_as_time_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNullable,

    // pub sqlx_types_time_time_as_time_not_null: postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,
    // pub sqlx_types_time_time_as_time_nullable: postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNullable,

    // pub sqlx_postgres_types_pg_interval_as_interval_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,
    // pub sqlx_postgres_types_pg_interval_as_interval_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNullable,

    // pub sqlx_types_time_date_as_date_not_null: postgresql_crud::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNotNull,
    // pub sqlx_types_time_date_as_date_nullable: postgresql_crud::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNullable,

    // pub sqlx_types_chrono_naive_date_as_date_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,
    // pub sqlx_types_chrono_naive_date_as_date_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNullable,

    // pub chrono_naive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,
    // pub chrono_naive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNullable,

    // pub sqlx_types_time_primitive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,
    // pub sqlx_types_time_primitive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNullable,

    // pub chrono_date_time_chrono_utc_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,
    // pub chrono_date_time_chrono_utc_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNullable,

    // pub chrono_date_time_chrono_local_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,
    // pub chrono_date_time_chrono_local_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNullable,

    // pub uuid_uuid_as_uuid_v4_init_by_postgres_not_null: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull,
    // // #[generate_postgresql_crud_primary_key] //todo maybe later support something else but now i think uuid v4 and v7 is enough
    // // pub uuid_uuid_as_uuid_v4_init_by_postgres_not_null_prime: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull,

    // pub uuid_uuid_as_uuid_init_by_client_not_null: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNotNull,
    // pub uuid_uuid_as_uuid_init_by_client_nullable: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNullable,

    // pub sqlx_types_ipnetwork_ip_network_as_inet_not_null: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_inet_nullable: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNullable,

    // pub sqlx_types_ipnetwork_ip_network_as_cidr_not_null: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_cidr_nullable: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNullable,

    // pub mac_address_mac_address_as_mac_addr_not_null: postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,
    // pub mac_address_mac_address_as_mac_addr_nullable: postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNullable,

    // pub sqlx_types_bit_vec_as_bit_not_null: postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNotNull,
    // pub sqlx_types_bit_vec_as_bit_nullable: postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNullable,

    // pub sqlx_types_bit_vec_as_varbit_not_null: postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNotNull,
    // pub sqlx_types_bit_vec_as_varbit_nullable: postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNullable,

    // pub pg_range_std_primitive_i32_as_int4_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,
    // pub pg_range_std_primitive_i32_as_int4_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNullable,

    // pub pg_range_std_primitive_i64_as_int8_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,
    // pub pg_range_std_primitive_i64_as_int8_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNullable,

    // pub pg_range_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,
    // pub pg_range_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNullable,

    // pub pg_range_sqlx_types_big_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,
    // pub pg_range_sqlx_types_big_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNullable,

    // pub pg_range_sqlx_types_time_date_as_date_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,
    // pub pg_range_sqlx_types_time_date_as_date_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNullable,

    // pub pg_range_chrono_naive_date_as_date_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,
    // pub pg_range_chrono_naive_date_as_date_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNullable,

    // pub pg_range_naive_date_time_as_timestamp_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNotNull,
    // pub pg_range_naive_date_time_as_timestamp_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNullable,

    // pub pg_range_time_primitive_date_time_as_timestamp_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNotNull,
    // pub pg_range_time_primitive_date_time_as_timestamp_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNullable,

    // pub pg_range_date_time_utc_as_timestamp_tz_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNotNull,
    // pub pg_range_date_time_utc_as_timestamp_tz_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNullable,

    // pub pg_range_date_time_local_as_timestamp_tz_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNotNull,
    // pub pg_range_date_time_local_as_timestamp_tz_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNullable,
    
    pub object_animal_as_jsonb_not_null: ObjectAnimalAsPostgresqlJsonbNotNull,
    // pub object_animal_as_jsonb_nullable: ObjectAnimalAsPostgresqlJsonbNullable,

    // pub option_object_animal_as_jsonb_not_null: StdOptionOptionObjectAnimalAsPostgresqlJsonbNotNull,
    // pub option_object_animal_as_jsonb_nullable: StdOptionOptionObjectAnimalAsPostgresqlJsonbNullable,

    // pub std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,
    // pub std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNullable,

    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,
    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNullable,

    ///////////////////////////
    // pub vec_std_primitive_i16_as_postgresql_int2_array_not_null: postgresql_crud::postgresql_type::VecStdPrimitiveI16AsPostgresqlInt2ArrayNotNull,
    //63max
}
//todo enum tree support
//todo generate wrapper type for all possible json type
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,postgresql_crud::GeneratePostgresqlJsonType)] //user type must implement utoipa::ToSchema trait
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
        postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i64_as_postgresql_big_serial_not_null_primary_key", true),
        postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull::create_table_column_query_part(&"std_primitive_i16_as_small_int2_not_null", false),
        postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2Nullable::create_table_column_query_part(&"std_primitive_i16_as_small_int2_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlInt4NotNull::create_table_column_query_part(&"std_primitive_i32_as_int4_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlInt4Nullable::create_table_column_query_part(&"std_primitive_i32_as_int4_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlInt8NotNull::create_table_column_query_part(&"std_primitive_i64_as_big_int8_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlInt8Nullable::create_table_column_query_part(&"std_primitive_i64_as_big_int8_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4NotNull::create_table_column_query_part(&"std_primitive_f32_as_float4_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4Nullable::create_table_column_query_part(&"std_primitive_f32_as_float4_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8NotNull::create_table_column_query_part(&"std_primitive_f64_as_float8_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8Nullable::create_table_column_query_part(&"std_primitive_f64_as_float8_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i16_as_small_serial_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i32_as_serial_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i64_as_big_serial_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull::create_table_column_query_part(&"sqlx_postgres_types_pg_money_as_money_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNullable::create_table_column_query_part(&"sqlx_postgres_types_pg_money_as_money_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNotNull::create_table_column_query_part(&"sqlx_types_decimal_as_numeric_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNullable::create_table_column_query_part(&"sqlx_types_decimal_as_numeric_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNotNull::create_table_column_query_part(&"sqlx_types_big_decimal_as_numeric_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNullable::create_table_column_query_part(&"sqlx_types_big_decimal_as_numeric_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull::create_table_column_query_part(&"std_primitive_bool_as_bool_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNullable::create_table_column_query_part(&"std_primitive_bool_as_bool_nullable", false),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNNotNull::create_table_column_query_part(&"std_string_string_as_char_n_not_null", false, postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNNullable::create_table_column_query_part(&"std_string_string_as_char_n_nullable", false, postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharNotNull::create_table_column_query_part(&"std_string_string_as_varchar_not_null", false, postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharNullable::create_table_column_query_part(&"std_string_string_as_varchar_nullable", false, postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlTextNotNull::create_table_column_query_part(&"std_string_string_as_text_not_null", false),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlTextNullable::create_table_column_query_part(&"std_string_string_as_text_nullable", false),
        // postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull::create_table_column_query_part(&"std_vec_vec_std_primitive_u8_as_bytea_not_null", false),
        // postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNullable::create_table_column_query_part(&"std_vec_vec_std_primitive_u8_as_bytea_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNotNull::create_table_column_query_part(&"sqlx_types_time_date_as_date_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNullable::create_table_column_query_part(&"sqlx_types_time_date_as_date_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull::create_table_column_query_part(&"sqlx_types_chrono_naive_date_as_date_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNullable::create_table_column_query_part(&"sqlx_types_chrono_naive_date_as_date_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull::create_table_column_query_part(&"sqlx_types_chrono_naive_time_as_time_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNullable::create_table_column_query_part(&"sqlx_types_chrono_naive_time_as_time_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNotNull::create_table_column_query_part(&"sqlx_types_time_time_as_time_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNullable::create_table_column_query_part(&"sqlx_types_time_time_as_time_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull::create_table_column_query_part(&"sqlx_postgres_types_pg_interval_as_interval_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNullable::create_table_column_query_part(&"sqlx_postgres_types_pg_interval_as_interval_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull::create_table_column_query_part(&"chrono_naive_date_time_as_timestamp_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNullable::create_table_column_query_part(&"chrono_naive_date_time_as_timestamp_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull::create_table_column_query_part(&"sqlx_types_time_primitive_date_time_as_timestamp_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNullable::create_table_column_query_part(&"sqlx_types_time_primitive_date_time_as_timestamp_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull::create_table_column_query_part(&"chrono_date_time_chrono_utc_as_timestamp_tz_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNullable::create_table_column_query_part(&"chrono_date_time_chrono_utc_as_timestamp_tz_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull::create_table_column_query_part(&"chrono_date_time_chrono_local_as_timestamp_tz_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNullable::create_table_column_query_part(&"chrono_date_time_chrono_local_as_timestamp_tz_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull::create_table_column_query_part(&"uuid_uuid_as_uuid_v4_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNotNull::create_table_column_query_part(&"uuid_uuid_as_uuid_init_by_client_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNullable::create_table_column_query_part(&"uuid_uuid_as_uuid_init_by_client_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_inet_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNullable::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_inet_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_cidr_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNullable::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_cidr_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull::create_table_column_query_part(&"mac_address_mac_address_as_mac_addr_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNullable::create_table_column_query_part(&"mac_address_mac_address_as_mac_addr_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNotNull::create_table_column_query_part(&"sqlx_types_bit_vec_as_bit_not_null", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNullable::create_table_column_query_part(&"sqlx_types_bit_vec_as_bit_nullable", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNotNull::create_table_column_query_part(&"sqlx_types_bit_vec_as_varbit_not_null", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNullable::create_table_column_query_part(&"sqlx_types_bit_vec_as_varbit_nullable", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull::create_table_column_query_part(&"pg_range_std_primitive_i32_as_int4_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNullable::create_table_column_query_part(&"pg_range_std_primitive_i32_as_int4_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull::create_table_column_query_part(&"pg_range_std_primitive_i64_as_int8_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNullable::create_table_column_query_part(&"pg_range_std_primitive_i64_as_int8_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNotNull::create_table_column_query_part(&"pg_range_naive_date_time_as_timestamp_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNullable::create_table_column_query_part(&"pg_range_naive_date_time_as_timestamp_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNotNull::create_table_column_query_part(&"pg_range_time_primitive_date_time_as_timestamp_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNullable::create_table_column_query_part(&"pg_range_time_primitive_date_time_as_timestamp_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNotNull::create_table_column_query_part(&"pg_range_date_time_utc_as_timestamp_tz_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNullable::create_table_column_query_part(&"pg_range_date_time_utc_as_timestamp_tz_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNotNull::create_table_column_query_part(&"pg_range_date_time_local_as_timestamp_tz_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNullable::create_table_column_query_part(&"pg_range_date_time_local_as_timestamp_tz_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull::create_table_column_query_part(&"pg_range_chrono_naive_date_as_date_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNullable::create_table_column_query_part(&"pg_range_chrono_naive_date_as_date_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull::create_table_column_query_part(&"pg_range_sqlx_types_time_date_as_date_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNullable::create_table_column_query_part(&"pg_range_sqlx_types_time_date_as_date_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull::create_table_column_query_part(&"pg_range_decimal_as_num_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNullable::create_table_column_query_part(&"pg_range_decimal_as_num_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull::create_table_column_query_part(&"pg_range_sqlx_types_big_decimal_as_num_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNullable::create_table_column_query_part(&"pg_range_sqlx_types_big_decimal_as_num_range_nullable", false),
        ObjectAnimalAsPostgresqlJsonbNotNull::create_table_column_query_part(&"object_animal_as_jsonb_not_null", false),
        // postgresql_crud::postgresql_type::VecStdPrimitiveI16AsPostgresqlInt2ArrayNotNull::create_table_column_query_part(&"vec_std_primitive_i16_as_postgresql_int2_array_not_null", false),
    );
    println!("{create_table_if_not_exists_query_stringified}");
    let _ = sqlx::query(&create_table_if_not_exists_query_stringified).execute(pool).await.unwrap();
}

//////////////////////////////////
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead,
    pub select: std::vec::Vec<ExampleSelect>,
}
#[derive(Debug)]
pub struct ReadOneParameters {
    pub payload: ReadOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadOneRouteLogicResponseVariants {
    Desirable(ExampleOptions),
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
    NotUniqueColumn {
        not_unique_column: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneRouteLogicErrorNamed> for TryReadOneRouteLogicResponseVariants {
    fn from(value: TryReadOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence } => Self::NotUniqueColumn { not_unique_column, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneRouteLogicErrorNamed {
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
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2580,
                        column: 264,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = ReadOneParameters {
        payload: match serde_json::from_slice::<ReadOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = ReadOnePayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryReadOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2652,
                            column: 249,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = {
        let mut increment: std::primitive::u64 = 0;
        format!(
            "select {} from example where {}",//std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = $1
            {
                let mut value = std::string::String::default();
                for element in &parameters.payload.select {
                    value.push_str(&match element {
                        ExampleSelect::StdPrimitiveI16AsSmallInt2NotNull(value) => <postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type_trait::PostgresqlType>::select_query_part(value, "std_primitive_i16_as_small_int2_not_null"),
                        ExampleSelect::StdPrimitiveI16AsSmallInt2Nullable(value) => <postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2Nullable as postgresql_crud::postgresql_type_trait::PostgresqlType>::select_query_part(value, "std_primitive_i16_as_small_int2_nullable"),
                        ExampleSelect::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(value) => {
                            <postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type_trait::PostgresqlType>::select_query_part(value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key")
                        }
                        ExampleSelect::ObjectAnimalAsJsonbNotNull(value) => <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type_trait::PostgresqlType>::select_query_part(value, "object_animal_as_jsonb_not_null"),
                    });
                    value.push_str(",");
                }
                let _ = value.pop();
                value
            },
            match <postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead as postgresql_crud::postgresql_type_trait::PostgresqlTypeSelfWhereFilter>::where_query_part(
                &parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                &mut increment,
                &"std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                false
            ) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryReadOneRouteLogicErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 2580,
                                column: 264,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            }
        )
    };
    println!("{}", query_string);
    let binded_query = {
        let query = sqlx::query::<sqlx::Postgres>(&query_string);
        let query = postgresql_crud::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2602,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2602,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => {
                    let mut std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<
                        postgresql_crud::Value<<postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type_trait::PostgresqlType>::Read>,
                    > = None;
                    let mut std_primitive_i16_as_small_int2_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNullRead>> = None;
                    let mut std_primitive_i16_as_small_int2_nullable: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NullableRead>> = None;
                    let mut object_animal_as_jsonb_not_null: std::option::Option<postgresql_crud::Value<ObjectAnimalAsPostgresqlJsonbNotNullRead>> = None;
                    for element in &parameters.payload.select {
                        match element {
                            ExampleSelect::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(_) => match sqlx::Row::try_get::<
                                <postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type_trait::PostgresqlType>::Read,
                                &std::primitive::str,
                            >(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key")
                            {
                                Ok(value) => {
                                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = Some(postgresql_crud::Value { value: value });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1248,
                                                column: 245,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                            ExampleSelect::StdPrimitiveI16AsSmallInt2NotNull(_) => match sqlx::Row::try_get::<postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNullRead, &std::primitive::str>(&value, "std_primitive_i16_as_small_int2_not_null") {
                                Ok(value) => {
                                    std_primitive_i16_as_small_int2_not_null = Some(postgresql_crud::Value { value: value });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1283,
                                                column: 245,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                            ExampleSelect::StdPrimitiveI16AsSmallInt2Nullable(_) => match sqlx::Row::try_get::<postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NullableRead, &std::primitive::str>(&value, "std_primitive_i16_as_small_int2_nullable") {
                                Ok(value) => {
                                    std_primitive_i16_as_small_int2_nullable = Some(postgresql_crud::Value { value: value });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1283,
                                                column: 245,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                            ExampleSelect::ObjectAnimalAsJsonbNotNull(_) => match sqlx::Row::try_get::<ObjectAnimalAsPostgresqlJsonbNotNullRead, &std::primitive::str>(&value, "object_animal_as_jsonb_not_null") {
                                Ok(value) => {
                                    object_animal_as_jsonb_not_null = Some(postgresql_crud::Value { value: value });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1283,
                                                column: 245,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                        }
                    }
                    ExampleOptions {
                        std_primitive_i16_as_small_int2_not_null,
                        std_primitive_i16_as_small_int2_nullable,
                        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                        object_animal_as_jsonb_not_null,
                    }
                }
                Err(error_0) => {
                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 4128,
                                column: 169,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        };
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneErrorNamed {
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
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_one_route_logic_error_named_with_serialize_deserialize: TryReadOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one(server_location: &std::primitive::str, parameters: ReadOneParameters) -> Result<ExampleOptions, TryReadOneErrorNamed> {
    let payload = {
        let value = ReadOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryReadOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2696,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/read_one", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryReadOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2748,
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
            return Err(TryReadOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2765,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryReadOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryReadOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2778,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_read_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryReadOneRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryReadOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryReadOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryReadOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryReadOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryReadOneRouteLogicResponseVariants::NotUniqueColumn { not_unique_column, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence },
        TryReadOneRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
    };
    Err(TryReadOneErrorNamed::TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_read_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2816,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ReadOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            select: postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn read_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(<ReadOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}



//////////////////////////////
