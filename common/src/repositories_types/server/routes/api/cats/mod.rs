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
pub struct CreateManyPayloadElement {
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdOptionOptionStdPrimitiveBool,
    pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdOptionOptionStdPrimitiveI16,
    pub std_primitive_i32_as_postgresql_int: postgresql_crud::StdOptionOptionStdPrimitiveI32,
}
#[derive(Debug)]
pub struct CreateManyPayload(pub std::vec::Vec<CreateManyPayloadElement>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElementWithSerializeDeserialize {
    std_primitive_bool_as_postgresql_bool:
        postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
    std_primitive_i16_as_postgresql_small_int:
        postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
    std_primitive_i32_as_postgresql_int:
        postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadWithSerializeDeserialize(
    pub std::vec::Vec<CreateManyPayloadElementWithSerializeDeserialize>,
);
impl std::convert::From<CreateManyPayloadElementWithSerializeDeserialize>
    for CreateManyPayloadElement
{
    fn from(value: CreateManyPayloadElementWithSerializeDeserialize) -> Self {
        let std_primitive_bool_as_postgresql_bool =
            postgresql_crud::StdOptionOptionStdPrimitiveBool::from(
                value.std_primitive_bool_as_postgresql_bool,
            );
        let std_primitive_i16_as_postgresql_small_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI16::from(
                value.std_primitive_i16_as_postgresql_small_int,
            );
        let std_primitive_i32_as_postgresql_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI32::from(
                value.std_primitive_i32_as_postgresql_int,
            );
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
        }
    }
}
impl std::convert::From<CreateManyPayloadWithSerializeDeserialize> for CreateManyPayload {
    fn from(value: CreateManyPayloadWithSerializeDeserialize) -> Self {
        let mut elements = std::vec::Vec::with_capacity(value.0.len());
        for element in value.0 {
            elements.push(CreateManyPayloadElement::from(element));
        }
        Self(elements)
    }
}
impl std::convert::From<CreateManyPayloadElement>
    for CreateManyPayloadElementWithSerializeDeserialize
{
    fn from(value: CreateManyPayloadElement) -> Self {
        let std_primitive_bool_as_postgresql_bool =
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                value.std_primitive_bool_as_postgresql_bool,
            );
        let std_primitive_i16_as_postgresql_small_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                value.std_primitive_i16_as_postgresql_small_int,
            );
        let std_primitive_i32_as_postgresql_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                value.std_primitive_i32_as_postgresql_int,
            );
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
        }
    }
}
impl std::convert::From<CreateManyPayload> for CreateManyPayloadWithSerializeDeserialize {
    fn from(value: CreateManyPayload) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| CreateManyPayloadElementWithSerializeDeserialize::from(element))
                .collect(),
        )
    }
}
#[derive(Debug)]
pub struct CreateManyParameters {
    pub payload: CreateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>),
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
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyRouteLogicErrorNamed>
    for TryCreateManyRouteLogicResponseVariants
{
    fn from(value: TryCreateManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            SerdeJson { serde_json, code_occurence } => Self :: SerdeJson
            { serde_json, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyRouteLogicErrorNamed {
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
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many_route_logic(
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
            let error = TryCreateManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2265,
                        column: 25,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryCreateManyRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    println!("kekw");
    let parameters = CreateManyParameters {
        payload: match serde_json::from_slice::<CreateManyPayloadWithSerializeDeserialize>(
            &body_bytes,
        ) {
            Ok(value) => {
                let value = CreateManyPayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryCreateManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 8262,
                            column: 9,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryCreateManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = {
        let mut increment: u64 = 0;
        let mut values = std::string::String::default();
        for element in &parameters.payload.0 {
            let mut value = std::string::String::default();
            match postgresql_crud::BindQuery::try_increment(
                &element.std_primitive_bool_as_postgresql_bool,
                &mut increment,
            ) {
                Ok(_) => {
                    value.push_str(&format!("${},", increment));
                }
                Err(_) => {
                    todo!()

                }
            }
            match postgresql_crud::BindQuery::try_increment(
                &element.std_primitive_i16_as_postgresql_small_int,
                &mut increment,
            ) {
                Ok(_) => {
                    value.push_str(&format!("${},", increment));
                }
                Err(_) => {
                    todo!()

                }
            }
            match postgresql_crud::BindQuery::try_increment(
                &element.std_primitive_i32_as_postgresql_int,
                &mut increment,
            ) {
                Ok(_) => {
                    value.push_str(&format!("${},", increment));
                }
                Err(_) => {
                    todo!()

                }
            }
            let _ = value.pop();
            values.push_str(&format!("({value}),"));
        }
        let _ = values.pop();
        format!("insert into dogs (std_primitive_bool_as_postgresql_bool, std_primitive_i16_as_postgresql_small_int, std_primitive_i32_as_postgresql_int) values {values} returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key")
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        for element in parameters.payload.0 {
            query = query.bind(element.std_primitive_bool_as_postgresql_bool.0);
            query = query.bind(element.std_primitive_i16_as_postgresql_small_int.0);
            query = query.bind(element.std_primitive_i32_as_postgresql_int.0);
        }
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2271,
                        column: 25,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryCreateManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2271,
                        column: 25,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryCreateManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        let mut rows = binded_query.fetch(pg_connection.as_mut());
        let mut results_vec = std::vec::Vec::new();
        while let Some(value) = {
            match {
                use futures::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 2208,
                                column: 25,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryCreateManyRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        } {
            match sqlx::Row::try_get::<std::primitive::i64, &std::primitive::str>(
                &value,
                "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
            ) {
                Ok(value) => {
                    results_vec.push(
                        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                            postgresql_crud::StdPrimitiveI64(value),
                        ),
                    );
                }
                Err(error_0) => {
                    let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 2208,
                                column: 25,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryCreateManyRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }
        results_vec
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryCreateManyRouteLogicResponseVariants::Desirable(value),
    ));
    *response.status_mut() = axum::http::StatusCode::CREATED;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
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
    TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_create_many_route_logic_error_named_with_serialize_deserialize:
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many(
    server_location: &std::primitive::str,
    parameters: CreateManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::StdPrimitiveI64>, TryCreateManyErrorNamed> {
    let payload = {
        let value = CreateManyPayloadWithSerializeDeserialize::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2350,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/create_many", server_location,);
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
            return Err(TryCreateManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2332,
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
            return Err(TryCreateManyErrorNamed::FailedToGetResponseText {
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
                        line: 2344,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryCreateManyRouteLogicResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TryCreateManyErrorNamed::DeserializeResponse {
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
                            line: 2338,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_create_many_route_logic_error_named_with_serialize_deserialize
    = match expected_response
    {
        TryCreateManyRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value =
            value.into_iter().map(| element | postgresql_crud::StdPrimitiveI64
            :: from(element)).collect(); return Ok(value);
        }, TryCreateManyRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryCreateManyRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        Postgresql { postgresql, code_occurence },
        TryCreateManyRouteLogicResponseVariants :: SerdeJson
        { serde_json, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize :: SerdeJson
        { serde_json, code_occurence },
        TryCreateManyRouteLogicResponseVariants :: CheckCommit
        { check_commit, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckCommit { check_commit, code_occurence },
        TryCreateManyRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }
    };
    Err(
        TryCreateManyErrorNamed::TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize {
            try_create_many_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6878,
                    column: 13,
                }),
            ),
        },
    )
}
