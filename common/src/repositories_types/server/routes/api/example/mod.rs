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
    pub std_primitive_i16_as_small_int2_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2Nullable,
    pub std_primitive_i16_as_small_int2_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull,

    pub std_primitive_i32_as_int4_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt4Nullable,
    pub std_primitive_i32_as_int4_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt4NotNull,

    pub std_primitive_i64_as_big_int8_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlInt8Nullable,
    pub std_primitive_i64_as_big_int8_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlInt8NotNull,

    pub std_primitive_f32_as_float4_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4Nullable,
    pub std_primitive_f32_as_float4_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4NotNull,

    pub std_primitive_f64_as_float8_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8Nullable,
    pub std_primitive_f64_as_float8_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8NotNull,

    // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i16_as_small_serial_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i32_as_serial_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlSerialNotNull,
    #[generate_postgresql_crud_primary_key]
    pub std_primitive_i64_as_big_serial_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialNotNull,

    pub sqlx_postgres_types_pg_money_as_money_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNullable,
    pub sqlx_postgres_types_pg_money_as_money_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

    pub sqlx_types_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNullable,
    pub sqlx_types_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNotNull,

    pub sqlx_types_big_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNullable,
    pub sqlx_types_big_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

    pub std_primitive_bool_as_bool_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNullable,
    pub std_primitive_bool_as_bool_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    // pub std_string_string_as_char_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNullable,
    // pub std_string_string_as_char_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNotNull,

    // pub std_string_string_as_char_n_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNNullable,
    // pub std_string_string_as_char_n_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNNotNull,

    // pub std_string_string_as_varchar_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharNullable,
    // pub std_string_string_as_varchar_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharNotNull,

    pub std_string_string_as_text_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlTextNullable,
    pub std_string_string_as_text_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlTextNotNull,

    // pub std_string_string_as_ci_text_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCiTextNullable,
    // pub std_string_string_as_ci_text_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCiTextNotNull,

    // pub sqlx_postgres_types_pg_ci_text_as_ci_text_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNullable,
    // pub sqlx_postgres_types_pg_ci_text_as_ci_text_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

    pub std_vec_vec_std_primitive_u8_as_bytea_nullable: postgresql_crud::postgresql_type::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNullable,
    pub std_vec_vec_std_primitive_u8_as_bytea_not_null: postgresql_crud::postgresql_type::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

    pub sqlx_types_time_date_as_date_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNullable,
    pub sqlx_types_time_date_as_date_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNotNull,

    pub sqlx_types_chrono_naive_date_as_date_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNullable,
    pub sqlx_types_chrono_naive_date_as_date_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

    pub sqlx_types_chrono_naive_time_as_time_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNullable,
    pub sqlx_types_chrono_naive_time_as_time_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

    pub sqlx_types_time_time_as_time_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNullable,
    pub sqlx_types_time_time_as_time_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    pub sqlx_postgres_types_pg_interval_as_interval_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNullable,
    pub sqlx_postgres_types_pg_interval_as_interval_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

    pub pg_range_std_primitive_i32_as_int4_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNullable,
    pub pg_range_std_primitive_i32_as_int4_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

    pub pg_range_std_primitive_i64_as_int8_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNullable,
    pub pg_range_std_primitive_i64_as_int8_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

    pub pg_range_naive_date_time_as_ts_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNullable,
    pub pg_range_naive_date_time_as_ts_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

    pub pg_range_time_primitive_date_time_as_ts_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNullable,
    pub pg_range_time_primitive_date_time_as_ts_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

    pub pg_range_date_time_utc_as_ts_tz_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNullable,
    pub pg_range_date_time_utc_as_ts_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

    pub pg_range_time_offset_date_time_as_ts_tz_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNullable,
    pub pg_range_time_offset_date_time_as_ts_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

    pub pg_range_date_time_local_as_ts_tz_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNullable,
    pub pg_range_date_time_local_as_ts_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

    pub pg_range_chrono_naive_date_as_date_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNullable,
    pub pg_range_chrono_naive_date_as_date_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

    pub pg_range_sqlx_types_time_date_as_date_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNullable,
    pub pg_range_sqlx_types_time_date_as_date_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

    pub pg_range_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNullable,
    pub pg_range_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

    pub pg_range_sqlx_types_big_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNullable,
    pub pg_range_sqlx_types_big_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

    pub chrono_naive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNullable,
    pub chrono_naive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

    pub sqlx_types_time_primitive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNullable,
    pub sqlx_types_time_primitive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

    pub time_offset_date_time_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNullable,
    pub time_offset_date_time_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

    pub chrono_date_time_chrono_utc_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNullable,
    pub chrono_date_time_chrono_utc_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

    pub chrono_date_time_chrono_local_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNullable,
    pub chrono_date_time_chrono_local_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

    pub sqlx_types_uuid_uuid_as_uuid_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidNullable,
    // #[generate_postgresql_crud_primary_key] //todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough //fails too but primary key is a different logic. need refactor it as different task
    pub sqlx_types_uuid_uuid_as_uuid_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidNotNull,

    pub std_net_ip_addr_as_inet_nullable: postgresql_crud::postgresql_type::postgresql_type::StdNetIpAddrAsPostgresqlInetNullable,
    pub std_net_ip_addr_as_inet_not_null: postgresql_crud::postgresql_type::postgresql_type::StdNetIpAddrAsPostgresqlInetNotNull,

    // pub sqlx_types_ipnetwork_ip_network_as_inet_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNullable,
    // pub sqlx_types_ipnetwork_ip_network_as_inet_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,

    // pub sqlx_types_ipnetwork_ip_network_as_cidr_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNullable,
    // pub sqlx_types_ipnetwork_ip_network_as_cidr_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    // todo mismatched types; Rust type `postgresql_crud_common::postgresql_type::postgresql_type::PostgresqlTypeSqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNullToRead` (as SQL type `INET`) is not compatible with SQL type `CIDR`s
    // pub sqlx_types_ipnetwork_ip_network_as_cidr_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNullable,
    // pub sqlx_types_ipnetwork_ip_network_as_cidr_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    pub mac_address_mac_address_as_mac_addr_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNullable,
    pub mac_address_mac_address_as_mac_addr_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

    // pub sqlx_types_bit_vec_as_bit_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNullable,
    // pub sqlx_types_bit_vec_as_bit_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNotNull,

    // pub sqlx_types_bit_vec_as_varbit_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNullable,
    pub sqlx_types_bit_vec_as_varbit_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNotNull,

    // pub object_animal_as_json_nullable: ObjectAnimalAsPostgresqlJson,
    // pub option_object_animal_as_json_nullable: StdOptionOptionObjectAnimalAsPostgresqlJsonNullable,
    // pub std_vec_vec_object_with_id_animal_as_json_nullable: StdVecVecObjectWithIdAnimalAsPostgresqlJsonNullable,
    // pub option_std_vec_vec_object_with_id_animal_as_json_nullable: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonNullable,

    // pub object_animal_as_json_not_null: ObjectAnimalAsPostgresqlJsonNotNull,
    // pub option_object_animal_as_json_not_null: StdOptionOptionObjectAnimalAsPostgresqlJsonNotNull,
    // pub std_vec_vec_object_with_id_animal_as_json_not_null: StdVecVecObjectWithIdAnimalAsPostgresqlJsonNotNull,
    // pub option_std_vec_vec_object_with_id_animal_as_json_not_null: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonNotNull,

    // pub object_animal_as_jsonb_nullable: ObjectAnimalAsPostgresqlJsonbNullable,
    // pub option_object_animal_as_jsonb_nullable: StdOptionOptionObjectAnimalAsPostgresqlJsonbNullable,
    // pub std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNullable,
    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNullable,

    // pub object_animal_as_jsonb_not_null: ObjectAnimalAsPostgresqlJsonbNotNull,
    // pub option_object_animal_as_jsonb_not_null: StdOptionOptionObjectAnimalAsPostgresqlJsonbNotNull,
    // pub std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,
    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,
    
    //63max
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