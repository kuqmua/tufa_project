#[derive(Debug, postgresql_crud::GeneratePostgresqlCrud)]
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
            check_commit: route_validators::check_commit::CheckCommitErrorNamed,
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
    // if let Err(error) = route_validators::check_commit::check_commit(
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
pub struct Dog {
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdPrimitiveBoolAsPostgresqlBool,
    // pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt,
    // pub std_primitive_i16_as_postgresql_small_int_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    // pub std_primitive_i16_as_postgresql_small_serial: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerial,
    // pub std_primitive_i16_as_postgresql_small_serial_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    // pub std_primitive_i16_as_postgresql_small_int2: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2,
    // pub std_primitive_i16_as_postgresql_small_int2_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2NotNull,

    pub std_primitive_i32_as_postgresql_int: postgresql_crud::StdPrimitiveI32AsPostgresqlInt,
    // pub std_primitive_i32_as_postgresql_int_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlIntNotNull,
    // pub std_primitive_i32_as_postgresql_serial: postgresql_crud::StdPrimitiveI32AsPostgresqlSerial,
    // pub std_primitive_i32_as_postgresql_serial_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlSerialNotNull,
    // pub std_primitive_i32_as_postgresql_int4: postgresql_crud::StdPrimitiveI32AsPostgresqlInt4,
    // pub std_primitive_i32_as_postgresql_int4_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlInt4NotNull,

    // pub std_primitive_i64_as_postgresql_big_int: postgresql_crud::StdPrimitiveI64AsPostgresqlBigInt,
    // pub std_primitive_i64_as_postgresql_big_int_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlBigIntNotNull,
    // pub std_primitive_i64_as_postgresql_big_serial: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerial,
    // pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
    // pub std_primitive_i64_as_postgresql_big_int8: postgresql_crud::StdPrimitiveI64AsPostgresqlInt8,
    // pub std_primitive_i64_as_postgresql_big_int8_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlInt8NotNull,

    // pub std_primitive_f32_as_postgresql_real: postgresql_crud::StdPrimitiveF32AsPostgresqlReal,
    // pub std_primitive_f32_as_postgresql_real_not_null: postgresql_crud::StdPrimitiveF32AsPostgresqlRealNotNull,
    // pub std_primitive_f32_as_postgresql_float4: postgresql_crud::StdPrimitiveF32AsPostgresqlFloat4,
    // pub std_primitive_f32_as_postgresql_float4_not_null: postgresql_crud::StdPrimitiveF32AsPostgresqlFloat4NotNull,

    // pub std_primitive_f64_as_postgresql_double_precision: postgresql_crud::StdPrimitiveF64AsPostgresqlDoublePrecision,
    // pub std_primitive_f64_as_postgresql_double_precision_not_null: postgresql_crud::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull,
    // pub std_primitive_f64_as_postgresql_float8: postgresql_crud::StdPrimitiveF64AsPostgresqlFloat8,
    // pub std_primitive_f64_as_postgresql_float8_not_null: postgresql_crud::StdPrimitiveF64AsPostgresqlFloat8NotNull,

    // pub std_string_string_as_postgresql_varchar: postgresql_crud::StdStringStringAsPostgresqlVarchar,
    // pub std_string_string_as_postgresql_varchar_not_null: postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull,
    // pub std_string_string_as_postgresql_char_n: postgresql_crud::StdStringStringAsPostgresqlCharN,
    // pub std_string_string_as_postgresql_char_n_not_null: postgresql_crud::StdStringStringAsPostgresqlCharNNotNull,
    // pub std_string_string_as_postgresql_text: postgresql_crud::StdStringStringAsPostgresqlText,
    // pub std_string_string_as_postgresql_text_not_null: postgresql_crud::StdStringStringAsPostgresqlTextNotNull,
    // pub std_string_string_as_postgresql_ci_text: postgresql_crud::StdStringStringAsPostgresqlCiText,
    // pub std_string_string_as_postgresql_ci_text_not_null: postgresql_crud::StdStringStringAsPostgresqlCiTextNotNull,

    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea: postgresql_crud::StdVecVecStdPrimitiveU8AsPostgresqlBytea,
    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea_not_null: postgresql_crud::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval: postgresql_crud::SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval_not_null: postgresql_crud::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_money_as_postgresql_money: postgresql_crud::SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
    // pub sqlx_postgres_types_pg_money_as_postgresql_money_not_null: postgresql_crud::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text: postgresql_crud::SqlxPostgresTypesPgCiTextAsPostgresqlCiText,
    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text_not_null: postgresql_crud::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

    // pub sqlx_types_big_decimal_as_postgresql_numeric: postgresql_crud::SqlxTypesBigDecimalAsPostgresqlNumeric,
    // pub sqlx_types_big_decimal_as_postgresql_numeric_not_null: postgresql_crud::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_decimal_as_postgresql_numeric: postgresql_crud::SqlxTypesDecimalAsPostgresqlNumeric,
    // pub sqlx_types_decimal_as_postgresql_numeric_not_null: postgresql_crud::SqlxTypesDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_not_null: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_not_null: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp: postgresql_crud::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_chrono_naive_date_as_postgresql_date: postgresql_crud::SqlxTypesChronoNaiveDateAsPostgresqlDate,
    // pub sqlx_types_chrono_naive_date_as_postgresql_date_not_null: postgresql_crud::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

    // pub sqlx_types_chrono_naive_time_as_postgresql_time: postgresql_crud::SqlxTypesChronoNaiveTimeAsPostgresqlTime,
    // pub sqlx_types_chrono_naive_time_as_postgresql_time_not_null: postgresql_crud::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz: postgresql_crud::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,
    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz_not_null: postgresql_crud::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull,

    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp: postgresql_crud::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz: postgresql_crud::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,
    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_not_null: postgresql_crud::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_time_date_as_postgresql_date: postgresql_crud::SqlxTypesTimeDateAsPostgresqlDate,
    // pub sqlx_types_time_date_as_postgresql_date_not_null: postgresql_crud::SqlxTypesTimeDateAsPostgresqlDateNotNull,

    // pub sqlx_types_time_time_as_postgresql_time: postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTime,
    // pub sqlx_types_time_time_as_postgresql_time_not_null: postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    // pub sqlx_types_uuid_uuid_as_postgresql_uuid: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuid,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNull,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough //fails too but primary key is a different logic. need refactor it as different task 

    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet_not_null: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr_not_null: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    // pub std_net_ip_addr_as_postgresql_inet: postgresql_crud::StdNetIpAddrAsPostgresqlInet,
    // pub std_net_ip_addr_as_postgresql_inet_not_null: postgresql_crud::StdNetIpAddrAsPostgresqlInetNotNull,
    // pub std_net_ip_addr_as_postgresql_cidr: postgresql_crud::StdNetIpAddrAsPostgresqlCidr,
    // pub std_net_ip_addr_as_postgresql_cidr_not_null: postgresql_crud::StdNetIpAddrAsPostgresqlCidrNotNull,

    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr: postgresql_crud::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr_not_null: postgresql_crud::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

    // pub sqlx_types_bit_vec_as_postgresql_bit: postgresql_crud::SqlxTypesBitVecAsPostgresqlBit,
    // pub sqlx_types_bit_vec_as_postgresql_bit_not_null: postgresql_crud::SqlxTypesBitVecAsPostgresqlBitNotNull,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit: postgresql_crud::SqlxTypesBitVecAsPostgresqlVarBit,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit_not_null: postgresql_crud::SqlxTypesBitVecAsPostgresqlVarBitNotNull,

    //todo what to do with generic?
    // pub sqlx_types_json_t_as_postgresql_json: postgresql_crud::SqlxTypesJsonTAsPostgresqlJson::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_not_null: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonNotNull::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_b: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonB::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonBNotNull::<Something>,//todo

    // pub serde_json_value_as_postgresql_json: postgresql_crud::SerdeJsonValueAsPostgresqlJson,
    // pub serde_json_value_as_postgresql_json_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonNotNull,
    // pub serde_json_value_as_postgresql_json_b: postgresql_crud::SerdeJsonValueAsPostgresqlJsonB,
    // pub serde_json_value_as_postgresql_json_b_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonBNotNull,
}

// #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
// pub struct Something {
//     something: std::string::String,
// }
///////////////////////
#[derive(Debug)]
pub struct ReadManyPayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        std::option::Option<std::vec::Vec<postgresql_crud::StdPrimitiveI64>>,
    pub std_primitive_bool_as_postgresql_bool:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveBool>>,
    pub std_primitive_i16_as_postgresql_small_int:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16>>,
    pub std_primitive_i32_as_postgresql_int:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32>>,
    pub select: std::vec::Vec<DogColumn>,
    pub order_by: postgresql_crud::OrderBy<DogColumn>,
    pub limit: postgresql_crud::StdPrimitiveI64,
    pub offset: postgresql_crud::StdPrimitiveI64,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadManyPayloadWithSerializeDeserialize {
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<
        std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>,
    >,
    std_primitive_bool_as_postgresql_bool: std::option::Option<
        std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
    >,
    std_primitive_i16_as_postgresql_small_int: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize>,
    >,
    std_primitive_i32_as_postgresql_int: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize>,
    >,
    select: std::vec::Vec<DogColumn>,
    order_by: postgresql_crud::OrderBy<DogColumn>,
    limit: postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
    offset: postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
}
impl std::convert::From<ReadManyPayloadWithSerializeDeserialize> for ReadManyPayload {
    fn from(value: ReadManyPayloadWithSerializeDeserialize) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            match value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| postgresql_crud::StdPrimitiveI64::from(element))
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_bool_as_postgresql_bool =
            match value.std_primitive_bool_as_postgresql_bool {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveBool::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_i16_as_postgresql_small_int =
            match value.std_primitive_i16_as_postgresql_small_int {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI32::from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        let select = value.select;
        let order_by = value.order_by;
        let limit = postgresql_crud::StdPrimitiveI64::from(value.limit);
        let offset = postgresql_crud::StdPrimitiveI64::from(value.offset);
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            select,
            order_by,
            limit,
            offset,
        }
    }
}
impl std::convert::From<ReadManyPayload> for ReadManyPayloadWithSerializeDeserialize {
    fn from(value: ReadManyPayload) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
        match
        value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize ::
            from(element)).collect :: < std :: vec :: Vec <
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize >> (),),
            None => None,
        };
        let std_primitive_bool_as_postgresql_bool = match
        value.std_primitive_bool_as_postgresql_bool
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
            :: from(element)).collect()), None => None
        };
        let std_primitive_i16_as_postgresql_small_int = match value
            .std_primitive_i16_as_postgresql_small_int
        {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize
            :: from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize
            :: from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        let select = value.select;
        let order_by = value.order_by;
        let limit = postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(value.limit);
        let offset = postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(value.offset);
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            select,
            order_by,
            limit,
            offset,
        }
    }
}
#[derive(Debug)]
pub struct ReadManyParameters {
    pub payload: ReadManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<DogOptions>),
    CheckBodySize {
        check_body_size:
            route_validators::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
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
        check_commit: route_validators::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
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
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBool {
        not_unique_std_primitive_bool_as_postgresql_bool: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallInt {
        not_unique_std_primitive_i16_as_postgresql_small_int: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlInt {
        not_unique_std_primitive_i32_as_postgresql_int: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyRouteLogicErrorNamed> for TryReadManyRouteLogicResponseVariants {
    fn from(value: TryReadManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            SerdeJson { serde_json, code_occurence } => Self :: SerdeJson
            { serde_json, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckedAdd { code_occurence } => Self :: CheckedAdd
            { code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniquePrimaryKey { not_unique_primary_key, code_occurence } =>
            Self :: NotUniquePrimaryKey
            { not_unique_primary_key, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniqueColumn { not_unique_column, code_occurence } => Self ::
            NotUniqueColumn { not_unique_column, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBool
            {
                not_unique_std_primitive_bool_as_postgresql_bool,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveBoolAsPostgresqlBool
            {
                not_unique_std_primitive_bool_as_postgresql_bool,
                code_occurence
            }, TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallInt
            {
                not_unique_std_primitive_i16_as_postgresql_small_int,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI16AsPostgresqlSmallInt
            {
                not_unique_std_primitive_i16_as_postgresql_small_int,
                code_occurence
            }, TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI32AsPostgresqlInt
            { not_unique_std_primitive_i32_as_postgresql_int, code_occurence }
            => Self :: NotUniqueStdPrimitiveI32AsPostgresqlInt
            { not_unique_std_primitive_i32_as_postgresql_int, code_occurence }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
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
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
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
        not_unique_primary_key: postgresql_crud::StdPrimitiveI64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBool {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool: postgresql_crud::StdPrimitiveBool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallInt {
        #[eo_to_std_string_string]
        not_unique_std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdPrimitiveI16,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlInt {
        #[eo_to_std_string_string]
        not_unique_std_primitive_i32_as_postgresql_int: postgresql_crud::StdPrimitiveI32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match route_validators::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error_0) => {
            let status_code =
                http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error_0);
            let error = TryReadManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 3504,
                        column: 25,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryReadManyRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = ReadManyParameters {
        payload: match serde_json::from_slice::<ReadManyPayloadWithSerializeDeserialize>(
            &body_bytes,
        ) {
            Ok(value) => {
                let value = ReadManyPayload::from(value);
                if let Some(std_primitive_i64_as_postgresql_big_serial_not_null_primary_key) =
                    &value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                {
                    let mut acc = std::vec::Vec::new();
                    for element in std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
                        if !acc.contains(&element) {
                            acc.push(&element);
                        } else {
                            let error_0 = *element;
                            let error =
                            TryReadManyRouteLogicErrorNamed :: NotUniquePrimaryKey
                            {
                                not_unique_primary_key : error_0, code_occurence :
                                error_occurence_lib :: code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 3108, column : 37,
                                }))
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_bool_as_postgresql_bool {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if let Some(value) = &element.value.0 {
                            if !acc.contains(&value) {
                                acc.push(&value);
                            } else {
                                let error_0 = postgresql_crud::StdPrimitiveBool(value.clone());
                                let error
                                = TryReadManyRouteLogicErrorNamed ::
                                NotUniqueStdPrimitiveBoolAsPostgresqlBool
                                {
                                    not_unique_std_primitive_bool_as_postgresql_bool : error_0,
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 3183, column : 37,
                                    }))
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_i16_as_postgresql_small_int {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if let Some(value) = &element.value.0 {
                            if !acc.contains(&value) {
                                acc.push(&value);
                            } else {
                                let error_0 = postgresql_crud::StdPrimitiveI16(value.clone());
                                let error =
                                TryReadManyRouteLogicErrorNamed ::
                                NotUniqueStdPrimitiveI16AsPostgresqlSmallInt
                                {
                                    not_unique_std_primitive_i16_as_postgresql_small_int :
                                    error_0, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 3183, column : 37,
                                    }))
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_i32_as_postgresql_int {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if let Some(value) = &element.value.0 {
                            if !acc.contains(&value) {
                                acc.push(&value);
                            } else {
                                let error_0 = postgresql_crud::StdPrimitiveI32(value.clone());
                                let error =
                                TryReadManyRouteLogicErrorNamed ::
                                NotUniqueStdPrimitiveI32AsPostgresqlInt
                                {
                                    not_unique_std_primitive_i32_as_postgresql_int : error_0,
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 3183, column : 37,
                                    }))
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                    }
                }
                let mut acc = std::vec::Vec::new();
                for element in &value.select {
                    if acc.contains(&element) {
                        let error_0 = *element;
                        let error = TryReadManyRouteLogicErrorNamed::NotUniqueColumn {
                            not_unique_column: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1481,
                                    column: 13,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryReadManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    } else {
                        acc.push(element);
                    }
                }
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
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 8192,
                            column: 9,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryReadManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = format!(
        "select {} from dogs {}",
        generate_query_vec_column(&parameters.payload.select),
        {
            let mut increment: u64 = 0;
            let mut additional_parameters = std::string::String::default();
            if let Some(_) = &parameters
                .payload
                .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "where",
                    false => " and",
                };
                match increment.checked_add(1) {
                    Some(value) => {
                        increment = value;
                    }
                    None => {
                        let error = TryReadManyRouteLogicErrorNamed::CheckedAdd {
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 3230,
                                    column: 29,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryReadManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                additional_parameters.push_str(& format!
            ("{} std_primitive_i64_as_postgresql_big_serial_not_null_primary_key in (select unnest(${}))",
            prefix, increment));
            }
            if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool {
                let prefix = match additional_parameters.is_empty() {
                    true => "where",
                    false => " and",
                };
                let bind_increments = {
                    let mut bind_increments = std::string::String::default();
                    for (index, element) in value.iter().enumerate() {
                        match postgresql_crud::BindQuery::try_generate_bind_increments(
                            element,
                            &mut increment,
                        ) {
                            Ok(value) => {
                                let handle =
                                    format!("std_primitive_bool_as_postgresql_bool ~ {value} ");
                                match index == 0 {
                                    true => {
                                        bind_increments.push_str(&handle);
                                    }
                                    false => {
                                        bind_increments.push_str(&format!(
                                            "{} {handle}",
                                            element.conjuctive_operator
                                        ));
                                    }
                                }
                            }
                            Err(error_0) => {
                                let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                            {
                                bind_query : error_0, code_occurence : error_occurence_lib
                                :: code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 3274, column : 29,
                                }))
                            };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                    if let false = bind_increments.is_empty() {
                        let _ = bind_increments.pop();
                    }
                    bind_increments
                };
                additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
            }
            if let Some(value) = &parameters.payload.std_primitive_i16_as_postgresql_small_int {
                let prefix = match additional_parameters.is_empty() {
                    true => "where",
                    false => " and",
                };
                let bind_increments = {
                    let mut bind_increments = std::string::String::default();
                    for (index, element) in value.iter().enumerate() {
                        match postgresql_crud::BindQuery::try_generate_bind_increments(
                            element,
                            &mut increment,
                        ) {
                            Ok(value) => {
                                let handle =
                                    format!("std_primitive_i16_as_postgresql_small_int ~ {value} ");
                                match index == 0 {
                                    true => {
                                        bind_increments.push_str(&handle);
                                    }
                                    false => {
                                        bind_increments.push_str(&format!(
                                            "{} {handle}",
                                            element.conjuctive_operator
                                        ));
                                    }
                                }
                            }
                            Err(error_0) => {
                                let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                            {
                                bind_query : error_0, code_occurence : error_occurence_lib
                                :: code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 3274, column : 29,
                                }))
                            };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                    if let false = bind_increments.is_empty() {
                        let _ = bind_increments.pop();
                    }
                    bind_increments
                };
                additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
            }
            if let Some(value) = &parameters.payload.std_primitive_i32_as_postgresql_int {
                let prefix = match additional_parameters.is_empty() {
                    true => "where",
                    false => " and",
                };
                let bind_increments = {
                    let mut bind_increments = std::string::String::default();
                    for (index, element) in value.iter().enumerate() {
                        match postgresql_crud::BindQuery::try_generate_bind_increments(
                            element,
                            &mut increment,
                        ) {
                            Ok(value) => {
                                let handle =
                                    format!("std_primitive_i32_as_postgresql_int ~ {value} ");
                                match index == 0 {
                                    true => {
                                        bind_increments.push_str(&handle);
                                    }
                                    false => {
                                        bind_increments.push_str(&format!(
                                            "{} {handle}",
                                            element.conjuctive_operator
                                        ));
                                    }
                                }
                            }
                            Err(error_0) => {
                                let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                            {
                                bind_query : error_0, code_occurence : error_occurence_lib
                                :: code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 3274, column : 29,
                                }))
                            };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                    if let false = bind_increments.is_empty() {
                        let _ = bind_increments.pop();
                    }
                    bind_increments
                };
                additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
            }
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                let value = &parameters.payload.order_by;
                let order_stringified = match &value.order {
                    Some(order) => order.to_string(),
                    None => postgresql_crud::Order::default().to_string(),
                };
                additional_parameters.push_str(&format!(
                    "{}order by {} {}",
                    prefix, value.column, order_stringified
                ));
            }
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                let value =
                    match postgresql_crud::BindQuery::try_generate_bind_increments(
                        &parameters.payload.limit,
                        &mut increment,
                    ) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                    {
                        bind_query : error_0, code_occurence : error_occurence_lib
                        :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 3337, column : 25,
                        }))
                    };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                            );
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
                let value =
                    match postgresql_crud::BindQuery::try_generate_bind_increments(
                        &parameters.payload.offset,
                        &mut increment,
                    ) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                    {
                        bind_query : error_0, code_occurence : error_occurence_lib
                        :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 3337, column : 25,
                        }))
                    };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    };
                additional_parameters.push_str(&format!("{}offset {}", prefix, value));
            }
            additional_parameters
        }
    );
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters
            .payload
            .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
        {
            query = query.bind(
                value
                    .into_iter()
                    .map(|element| element.into_inner().clone())
                    .collect::<std::vec::Vec<std::primitive::i64>>(),
            );
        }
        if let Some(values) = parameters.payload.std_primitive_bool_as_postgresql_bool {
            for value in values {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
        }
        if let Some(values) = parameters.payload.std_primitive_i16_as_postgresql_small_int {
            for value in values {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
        }
        if let Some(values) = parameters.payload.std_primitive_i32_as_postgresql_int {
            for value in values {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
        }
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.limit, query);
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.offset, query);
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
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 3510,
                        column: 25,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryReadManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
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
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 3510,
                        column: 25,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryReadManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        let mut rows = binded_query.fetch(executor.as_mut());
        let mut results_vec = std::vec::Vec::new();
        // let wrapper_vec_column = WrapperVecColumn(parameters.payload.select);
        while let Some(row) = match {
            use futures::TryStreamExt;
            rows.try_next()
        }
        .await
        {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 3452,
                            column: 25,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryReadManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        } {
            //
            //
            let mut
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key : std
            :: option :: Option < postgresql_crud :: Value <
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize > > = None;
            let mut std_primitive_bool_as_postgresql_bool: std::option::Option<
                postgresql_crud::Value<
                    postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
                >,
            > = None;
            let mut std_primitive_i16_as_postgresql_small_int: std::option::Option<
                postgresql_crud::Value<
                    postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
                >,
            > = None;
            let mut std_primitive_i32_as_postgresql_int: std::option::Option<
                postgresql_crud::Value<
                    postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
                >,
            > = None;
            for element in &parameters.payload.select {
                match element {
                    DogColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => match sqlx::Row::try_get::<std::option::Option<std::primitive::i64>, &std::primitive::str>(&row, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
                        Ok(value) => {
                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = value.map(|value| {
                                postgresql_crud::Value {
                                    value: postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(postgresql_crud::StdPrimitiveI64(value))
                                }
                            });
                        },
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from(
                                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                        ),
                                        line: 3452,
                                        column: 25,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                                TryReadManyRouteLogicResponseVariants::from(error),
                            ));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                    // {
                    //     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = {
                    //         let value: std::option::Option<std::primitive::i64> = row.try_get(
                    //             "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                    //         )?;
                    //         value.map(|value| postgresql_crud::Value {
                    //             value: postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                    //                 postgresql_crud::StdPrimitiveI64(value),
                    //             ),
                    //         })
                    //     };
                    // }
                    DogColumn::StdPrimitiveBoolAsPostgresqlBool => match sqlx::Row::try_get::<std::option::Option<std::option::Option<std::primitive::bool>>, &std::primitive::str>(&row, "std_primitive_bool_as_postgresql_bool") {
                        Ok(value) => {
                            std_primitive_bool_as_postgresql_bool = value.map(|value| {
                                postgresql_crud::Value {
                                    value: postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(postgresql_crud::StdOptionOptionStdPrimitiveBool(value))
                                }
                            });
                        },
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from(
                                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                        ),
                                        line: 3452,
                                        column: 25,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                                TryReadManyRouteLogicResponseVariants::from(error),
                            ));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                    DogColumn::StdPrimitiveI16AsPostgresqlSmallInt => match sqlx::Row::try_get::<std::option::Option<std::option::Option<std::primitive::i16>>, &std::primitive::str>(&row, "std_primitive_i16_as_postgresql_small_int") {
                        Ok(value) => {
                            std_primitive_i16_as_postgresql_small_int = value.map(|value| {
                                postgresql_crud::Value {
                                    value: postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(postgresql_crud::StdOptionOptionStdPrimitiveI16(value))
                                }
                            });
                        },
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from(
                                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                        ),
                                        line: 3452,
                                        column: 25,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                                TryReadManyRouteLogicResponseVariants::from(error),
                            ));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                    DogColumn::StdPrimitiveI32AsPostgresqlInt => match sqlx::Row::try_get::<std::option::Option<std::option::Option<std::primitive::i32>>, &std::primitive::str>(&row, "std_primitive_i32_as_postgresql_int") {
                        Ok(value) => {
                            std_primitive_i32_as_postgresql_int = value.map(|value| {
                                postgresql_crud::Value {
                                    value: postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(postgresql_crud::StdOptionOptionStdPrimitiveI32(value))
                                }
                            });
                        },
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from(
                                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                        ),
                                        line: 3452,
                                        column: 25,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                                TryReadManyRouteLogicResponseVariants::from(error),
                            ));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            results_vec.push(DogOptions {
                std_primitive_bool_as_postgresql_bool,
                std_primitive_i16_as_postgresql_small_int,
                std_primitive_i32_as_postgresql_int,
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            });

            //
            //
            // match wrapper_vec_column.options_try_from_sqlx_row(&row) {
            //     Ok(value) => {
            //         results_vec.push(value);
            //     }
            //     Err(error_0) => {
            //         let error = TryReadManyRouteLogicErrorNamed::Postgresql {
            //             postgresql: error_0,
            //             code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            //                 file!().to_owned(),
            //                 line!(),
            //                 column!(),
            //                 Some(error_occurence_lib::code_occurence::MacroOccurence {
            //                     file: std::string::String::from(
            //                         "postgresql_crud/generate_postgresql_crud/src/lib.rs",
            //                     ),
            //                     line: 3452,
            //                     column: 25,
            //                 }),
            //             ),
            //         };
            //         eprintln!("{error}");
            //         let mut response = axum::response::IntoResponse::into_response(axum::Json(
            //             TryReadManyRouteLogicResponseVariants::from(error),
            //         ));
            //         *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            //         return response;
            //     }
            // }
        }
        results_vec
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryReadManyRouteLogicResponseVariants::Desirable(value),
    ));
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
        not_unique_primary_key: postgresql_crud::StdPrimitiveI64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBool {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool: postgresql_crud::StdPrimitiveBool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallInt {
        #[eo_to_std_string_string]
        not_unique_std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdPrimitiveI16,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlInt {
        #[eo_to_std_string_string]
        not_unique_std_primitive_i32_as_postgresql_int: postgresql_crud::StdPrimitiveI32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_many_route_logic_error_named_with_serialize_deserialize:
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many(
    server_location: &std::primitive::str,
    parameters: ReadManyParameters,
) -> Result<std::vec::Vec<DogOptions>, TryReadManyErrorNamed> {
    let payload = {
        if let Some(value) = &parameters
            .payload
            .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
        {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if !acc.contains(&element) {
                    acc.push(&element);
                } else {
                    let error_0 = *element;
                    return Err(TryReadManyErrorNamed::NotUniquePrimaryKey {
                        not_unique_primary_key: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 3549,
                                column: 33,
                            }),
                        ),
                    });
                }
            }
        }
        if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if let Some(value) = &element.value.0 {
                    if !acc.contains(&value) {
                        acc.push(&value);
                    } else {
                        return
                        Err(TryReadManyErrorNamed ::
                        NotUniqueStdPrimitiveBoolAsPostgresqlBool
                        {
                            not_unique_std_primitive_bool_as_postgresql_bool :
                            postgresql_crud::StdPrimitiveBool(value.clone()),
                            code_occurence : error_occurence_lib :: code_occurence ::
                            CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 3590, column : 37,
                            })),
                        });
                    }
                }
            }
        }
        if let Some(value) = &parameters.payload.std_primitive_i16_as_postgresql_small_int {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if let Some(value) = &element.value.0 {
                    if !acc.contains(&value) {
                        acc.push(&value);
                    } else {
                        return
                        Err(TryReadManyErrorNamed ::
                        NotUniqueStdPrimitiveI16AsPostgresqlSmallInt
                        {
                            not_unique_std_primitive_i16_as_postgresql_small_int :
                            postgresql_crud::StdPrimitiveI16(value.clone()),
                            code_occurence : error_occurence_lib :: code_occurence ::
                            CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 3590, column : 37,
                            })),
                        });
                    }
                }
            }
        }
        if let Some(value) = &parameters.payload.std_primitive_i32_as_postgresql_int {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if let Some(value) = &element.value.0 {
                    if !acc.contains(&value) {
                        acc.push(&value);
                    } else {
                        return
                        Err(TryReadManyErrorNamed ::
                        NotUniqueStdPrimitiveI32AsPostgresqlInt
                        {
                            not_unique_std_primitive_i32_as_postgresql_int :
                            postgresql_crud::StdPrimitiveI32(value.clone()),
                            code_occurence : error_occurence_lib :: code_occurence ::
                            CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 3590, column : 37,
                            })),
                        });
                    }
                }
            }
        }
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.select {
            if acc.contains(&element) {
                let error_0 = *element;
                return Err(TryReadManyErrorNamed::NotUniqueColumn {
                    not_unique_column: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1502,
                            column: 13,
                        }),
                    ),
                });
            } else {
                acc.push(element);
            }
        }
        let value = ReadManyPayloadWithSerializeDeserialize::from(parameters.payload);
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
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 3677,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/read_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(
            &postgresql_crud::CommitSnakeCase.to_string(),
            git_info::PROJECT_GIT_INFO.commit,
        )
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
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 3659,
                        column: 21,
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
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 3671,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryReadManyRouteLogicResponseVariants>(&error_2) {
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
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 3665,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_read_many_route_logic_error_named_with_serialize_deserialize =
    match expected_response
    {
        TryReadManyRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value =
            value.into_iter().map(| element | DogOptions ::
            from(element)).collect(); return Ok(value);
        }, TryReadManyRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryReadManyRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: Postgresql
        { postgresql, code_occurence }, TryReadManyRouteLogicResponseVariants
        :: SerdeJson { serde_json, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: SerdeJson
        { serde_json, code_occurence }, TryReadManyRouteLogicResponseVariants
        :: CheckCommit { check_commit, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: CheckCommit
        { check_commit, code_occurence },
        TryReadManyRouteLogicResponseVariants :: CheckedAdd { code_occurence }
        => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckedAdd { code_occurence }, TryReadManyRouteLogicResponseVariants
        :: BindQuery { bind_query, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: BindQuery
        { bind_query, code_occurence }, TryReadManyRouteLogicResponseVariants
        :: NotUniquePrimaryKey { not_unique_primary_key, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryReadManyRouteLogicResponseVariants :: NotUniqueColumn
        { not_unique_column, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniqueColumn { not_unique_column, code_occurence },
        TryReadManyRouteLogicResponseVariants ::
        NotUniqueStdPrimitiveBoolAsPostgresqlBool
        { not_unique_std_primitive_bool_as_postgresql_bool, code_occurence }
        => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniqueStdPrimitiveBoolAsPostgresqlBool
        { not_unique_std_primitive_bool_as_postgresql_bool, code_occurence },
        TryReadManyRouteLogicResponseVariants ::
        NotUniqueStdPrimitiveI16AsPostgresqlSmallInt
        {
            not_unique_std_primitive_i16_as_postgresql_small_int,
            code_occurence
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniqueStdPrimitiveI16AsPostgresqlSmallInt
        {
            not_unique_std_primitive_i16_as_postgresql_small_int,
            code_occurence
        }, TryReadManyRouteLogicResponseVariants ::
        NotUniqueStdPrimitiveI32AsPostgresqlInt
        { not_unique_std_primitive_i32_as_postgresql_int, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniqueStdPrimitiveI32AsPostgresqlInt
        { not_unique_std_primitive_i32_as_postgresql_int, code_occurence }
    };
    Err(
        TryReadManyErrorNamed::TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
            try_read_many_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6808,
                    column: 13,
                }),
            ),
        },
    )
}
